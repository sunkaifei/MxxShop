//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::PageRequest;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, web};
use actix_web_grants::protect;
use crate::core::permission::jwt_util::JWTToken;
use crate::core::web::base_controller::get_user;

use crate::core::web::entity::common::{BathIdRequest, InfoId};
use crate::core::web::response::*;
use crate::modules::system::entity::role_model::*;
use crate::modules::system::service::{admin_service, role_service};

// 添加角色信息
#[post("/system/role/save")]
#[protect("role:save")]
pub async fn role_save(req: HttpRequest, item: web::Json<RoleSaveRequest>) -> HttpResponse {
    //log::info!("role_save params: {:?}", &item);
    let mut sys_role:RoleDTO = item.0.into();
    if sys_role.role_name.as_ref().map_or(true, |name| name.trim().is_empty()) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("角色名称不能为空".to_string()));
    }
    //获取用户信息
    let jwt_token:JWTToken = get_user(req).unwrap_or_default();
    let admin = match admin_service::get_by_detail(&jwt_token.id).await {
        Ok(op_admin) => {
            match op_admin {
                Some(admin) => admin,
                None => {
                    return HttpResponse::Ok().json(ResVO::<String>::error_msg("当前管理员信息不存在，请联系超级管理员".to_string()));
                }
            }
        },
        Err(_) => {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("获取当前管理员信息错误".to_string()));
        }
    };
    sys_role.admin = admin;
    let result = role_service::save_role(&sys_role).await;
    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
}

// 删除角色信息
#[delete("/system/role/bath_delete")]
#[protect("role:delete")]
pub async fn bath_delete_role(item: web::Json<BathIdRequest>) -> HttpResponse {
    let delete_role = item.0;
    if let Some(role_ids) = &delete_role.ids {
        if !role_ids.is_empty() {
            let filtered_ids: Vec<Option<String>> = role_ids
                .iter()
                .filter_map(|item| {
                    item.as_ref().map(|s| s.trim().is_empty()).map(|trimmed| {
                        if !trimmed {
                            Some(item.clone())
                        } else {
                            None
                        }
                    }).flatten()
                })
                .collect();
            if !filtered_ids.is_empty() {
                let result = role_service::delete_in_column(filtered_ids).await;
                return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
            }
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("未获取到删除的角色ID".to_string()));
        } else {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("未获取到删除的角色ID".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("请选择要删除的角色".to_string()));
    }
}

// 更新角色信息
#[put("/system/role/update")]
#[protect("role:update")]
pub async fn update_role(req: HttpRequest, item: web::Json<RoleUpdateRequest>) -> HttpResponse {
    //log::info!("role_update params: {:?}", &item);
    let mut sys_role:RoleDTO = item.0.into();
    //获取用户信息
    let jwt_token:JWTToken = get_user(req).unwrap_or_default();
    let admin = match admin_service::get_by_detail(&jwt_token.id).await {
        Ok(op_admin) => {
            match op_admin {
                Some(admin) => admin,
                None => {
                    return HttpResponse::Ok().json(ResVO::<String>::error_msg("当前管理员信息不存在，请联系超级管理员".to_string()));
                }
            }
        },
        Err(_) => {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("获取当前管理员信息错误".to_string()));
        }
    };
    sys_role.admin = admin;
    let result = role_service::update_role(&sys_role).await;
    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
}

// 更新角色关联的菜单
// pub async fn update_role_menu(item: web::Json<UpdateRoleMenuRequest>) -> HttpResponse {
//     //log::info!("update_role_menu params: {:?}", &item);
//     let sys_role = item.0;
//
//     let result = role_service::insert_batch(sys_role).await;
//     return result;
// }

#[get("/system/role/detail/{id}")]
pub async fn get_role_detail(item: web::Path<InfoId>) -> HttpResponse {
    if item.id.clone().is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("角色id不能为空".to_string()));
    }
    return match role_service::get_by_detail(&item.id).await {
        Ok(role_op) => match role_op {
            None => {
                HttpResponse::Ok().json(ResVO::<String>::error_msg("角色信息不存在".to_string()))
            }
            Some(role) => {
                //查询角色关联的菜单ID
                let role_menu_ids = role_service::get_merge_by_role_id(&item.id).await.unwrap_or_default();
                let role_detail: RoleDetail = role.into();
                let role_vo = RoleDetailVO{
                    menu_ids: role_menu_ids,
                    role: role_detail,
                };

                HttpResponse::Ok().json(ResVO::ok_with_data(role_vo))
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

// 查询角色列表
#[get("/system/role/list")]
pub async fn role_list(item: web::Query<RoleListRequest>) -> HttpResponse {
    //log::info!("role_list params: {:?}", &item);

    let role_name = item.role_name.clone().unwrap_or_default();
    let status = item.status.clone().unwrap_or_default();

    let page_req = PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let result = role_service::page(&role_name, &status, page_req).await;


    return match result {
        Ok(page) => {
            let mut role_list: Vec<RoleListData> = Vec::new();
            for role in page.records {
                role_list.push(RoleListData {
                    id: role.id,
                    sort: role.sort,
                    status: role.status,
                    role_name: role.role_name,
                    remark: role.remark,
                    update_time: role.update_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
                })
            }

            let page_data = ResultPage {
                current_page: page.page_no,
                list: role_list,
                total: page.total,
            };
            HttpResponse::Ok().json(ok_result_page(page_data))
        }
        Err(err) => {
            HttpResponse::Ok().json(err_result_page(err.to_string()))
        }
    };
}