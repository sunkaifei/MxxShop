//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use crate::core::web::entity::common::BathIdRequest;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, web};
use crate::core::permission::jwt_util::JWTToken;

use crate::core::web::base_controller::get_user;
use crate::core::web::entity::common::{InfoId};
use crate::core::web::response::{ResVO};
use crate::modules::system::entity::admin_entity::SystemAdmin;
use crate::modules::system::entity::menus_model::{MenuSaveRequest, MenuUpdateRequest, SystemMenuResponse, MenuAndRoleResponse, RoleMenuRoutes, RoleMenuResponse};
use crate::modules::system::entity::role_entity::SystemRole;
use crate::modules::system::service::{admin_service, menus_service, role_service};


// 添加菜单
#[post("/system/menu/save")]
pub async fn menu_save(item: web::Json<MenuSaveRequest>) -> HttpResponse {
    let sys_menu = item.0;
    if sys_menu.menu_name.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单名称不能为空".to_string()))
    }
    if menus_service::find_by_name_unique(&sys_menu.menu_name, &sys_menu.parent_id, &None).await.unwrap_or_default() == true{
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单名称不能重复".to_string()))
    }
    if menus_service::find_by_path_unique(&sys_menu.path, &None).await.unwrap_or_default() == true{
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单的路由不能重复".to_string()))
    }
    if menus_service::find_by_perms_unique(&sys_menu.perms, &None).await.unwrap_or_default() == true{
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单的权限标识不能重复".to_string()))
    }
    let result = menus_service::add_menu(sys_menu).await;
    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result))
}

// 更新菜单
#[put("/system/menu/update")]
pub async fn menu_update(item: web::Json<MenuUpdateRequest>) -> HttpResponse {
    let sys_menu = item.0;
    if sys_menu.menu_name.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单名称不能为空".to_string()))
    }
    if menus_service::find_by_name_unique(&sys_menu.menu_name, &sys_menu.parent_id, &sys_menu.id).await.unwrap_or_default() == true{
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单名称不能重复".to_string()))
    }
    if menus_service::find_by_path_unique(&sys_menu.path, &sys_menu.id).await.unwrap_or_default() == true{
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单的路由不能重复".to_string()))
    }
    if menus_service::find_by_perms_unique(&sys_menu.perms, &sys_menu.id).await.unwrap_or_default() == true{
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("菜单的权限标识不能重复".to_string()))
    }
    let result = menus_service::update_menu(sys_menu).await;

    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result))
}

// 删除菜单信息
#[delete("/system/menu/bath_delete")]
pub async fn menu_delete(item: web::Json<BathIdRequest>) -> HttpResponse {
    //log::info!("menu_delete params: {:?}", &item);
    if let Some(ids_vec) = item.ids.clone() {
        if ids_vec.is_empty() {
            HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()))
        } else {
            return menus_service::delete_in_column(ids_vec).await;
        }
    }else {
        HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()))
    }
}

#[get("/system/menu/detail/{id}")]
pub async fn menu_detail(path: web::Path<InfoId>) -> HttpResponse {
    if path.id.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("ID不能为空".to_string()));
    }
    let result = menus_service::get_by_detail(&path.id).await;
    let menu: SystemMenuResponse = match result {
        Ok(Some(v)) => { v.clone().into()},
        Ok(None) => {
            return HttpResponse::Ok().json("菜单不存在".to_string());
        },
        Err(err) => {
            return HttpResponse::Ok().json(err.to_string());
        }
    };

    let role_id_list = role_service::query_menu_by_role(menu.id).await;


    let menu_role_result = MenuAndRoleResponse {
        menu,
        role_ids: role_id_list,
    };
    return HttpResponse::Ok().json(ResVO::ok_with_data(menu_role_result));
}

#[get("/system/menu_list")]
pub async fn menu_list() -> HttpResponse {
    // 菜单是树形结构不需要分页
    let result = menus_service::all_menu_list_tree().await;
    return match result {
        Ok(router_list) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(router_list))
        }
        Err(err) => {
            log::error!("获取菜单列表错误: {:?}", &err);
            HttpResponse::Ok().json(ResVO::<String>::error_msg("获取菜单列表错误".to_string()))
        }
    }
}

#[get("/system/menu/getParams")]
pub async fn get_menu_params() -> HttpResponse {

    let menu_result = menus_service::select_all().await;
    let mut menus: Vec<SystemMenuResponse> = Vec::new();
    match menu_result {
        Ok(other_menu_list) => {
            for menu in other_menu_list {
                menus.push(menu.clone().into());
            }
        }
        Err(err) => {
            log::error!("查询菜单异常,{}", &err.to_string());
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("查询菜单异常,".to_string() + &err.to_string()));
        }
    };

    let role_result = role_service::select_all().await;
    let roles : Vec<SystemRole> = match role_result {
        Ok(v) => v.clone(),
        Err(err) => {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("查询权限组异常,".to_string() + &err.to_string()));
        }
    };
    let get_menus_roles = RoleMenuResponse {
        menus,
        roles,
    };
    return HttpResponse::Ok().json(ResVO::ok_with_data(get_menus_roles));
}


///重新获取用户权限和路由
#[get("/system/menu/getUserMenus")]
pub async fn get_user_menu(req: HttpRequest) -> HttpResponse {
    //获取用户信息
    let jwt_token:JWTToken = get_user(req).unwrap_or_default();
    let admin_info = match admin_service::get_by_detail(&jwt_token.id).await {
        Ok(Some(admin_op)) => admin_op,
        _ => {
            SystemAdmin::default()
        }
    };
    if admin_info == SystemAdmin::default() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("管理员信息不存在".to_string()));
    }
    //判断是否是管理员
    let is_admin = admin_info.user_type == Option::from(1);
    //根据id查询路由
    let result = menus_service::get_router_tree(&is_admin, &jwt_token.id).await;
    return match result {
        Ok(v) => {

            //查询用户所在权限组
            let user_role: Vec<String> = match menus_service::query_btn_menu(&is_admin, &jwt_token.id).await {
                Ok(role_list) => role_list.clone(),
                Err(err) => {
                    return HttpResponse::Ok().json(ResVO::<String>::error_msg(format!("权限组查询失败, {}", err)));
                }
            };
            let menu_data = RoleMenuRoutes {
                permissions: user_role,
                menu_list: v.clone(),
            };
            HttpResponse::Ok().json(ResVO::ok_with_data(menu_data))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(
                "查询菜单异常,".to_string() + &err.to_string()
            ))
        }
    };
}