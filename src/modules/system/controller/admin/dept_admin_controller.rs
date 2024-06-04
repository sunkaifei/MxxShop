//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_web::{delete, get, HttpResponse, post, put, web};
use actix_web_grants::protect;
use crate::core::web::entity::common::{BathIdRequest, InfoId};

use crate::core::web::response::{ResVO};
use crate::modules::system::entity::dept_model::{DeptPageRequest, DeptSaveRequest, DeptUpdateRequest, SystemDeptVO};
use crate::modules::system::service::{dept_service};

#[post("/system/dept/save")]
#[protect("dept:save")]
pub async fn dept_save(item: web::Json<DeptSaveRequest>) -> HttpResponse {
    //log::info!("dept_save params: {:?}", &item);

    let sys_dept = item.0;
    if let Some(dept_name) = &sys_dept.dept_name {
        if dept_name.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("部门名称不能为空".to_string()));
        } else if dept_name.len() > 30 {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("部门名称不能超过30个字符".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("部门名称不能为空".to_string()));
    }
    let result = dept_service::add_dept(sys_dept).await;

    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
}

// 删除部门信息
#[delete("/system/dept/batch_delete")]
pub async fn dept_batch_delete(item: web::Json<BathIdRequest>) -> HttpResponse {
    //log::info!("dept_delete params: {:?}", &item);
    let ids = item.0;
    let result = dept_service::delete_in_column(ids).await;
    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
}

///更新部门信息
#[put("/system/dept/update")]
#[protect("dept:update")]
pub async fn dept_update(item: web::Json<DeptUpdateRequest>) ->  HttpResponse {
    //log::info!("dept_update params: {:?}", &item);
    let sys_dept = item.0;
    if let Some(dept_name) = &sys_dept.dept_name {
        if dept_name.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("部门名称不能为空".to_string()));
        } else if dept_name.len() > 30 {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("部门名称不能超过30个字符".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("部门名称不能为空".to_string()));
    }
    let result = dept_service::update_dept(sys_dept).await;
    return match result {
        Ok(v) => {
            if v == 0 {
                return HttpResponse::Ok().json(ResVO::<String>::error_msg("更新部门信息异常".to_string()));
            }
            HttpResponse::Ok().json(ResVO::ok_with_data(v))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg("更新部门信息异常,".to_string() + &err.to_string()))
        }
    }
}

#[get("/system/dept/tree")]
pub async fn query_dept_tree() -> HttpResponse {
    //log::info!("query_user_menu params: {:?}", auth);
    let result = dept_service::all_dept_list_tree().await;
    return match result {
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(v))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(
                "查询部门列表树异常,".to_string() + &err.to_string()
            ))
        }
    };
}

#[get("/system/dept/detail/{id}")]
#[protect("dept:detail:show")]
pub async fn get_by_detail(item: web::Path<InfoId>) -> HttpResponse {
    if item.id.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("部门id不能为空".to_string()));
    }
    return match dept_service::get_by_detail(&item.id).await {
        Ok(dept_op) => match dept_op {
            None => {
                HttpResponse::Ok().json(ResVO::<String>::error_msg("部门信息不存在".to_string()))
            }
            Some(dept_entity) => {
                let dept_detail: SystemDeptVO = dept_entity.into();
                HttpResponse::Ok().json(ResVO::ok_with_data(dept_detail))
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

// 查询用户列表
#[get("/system/dept/list")]
#[protect("dept:list:show")]
pub async fn dept_list(item: web::Query<DeptPageRequest>) -> HttpResponse {
    let dept_request = item.0;
    let result = dept_service::select_all_list(dept_request).await;
    return match result {
        Ok(system_dept_vec) => {
            // 将 Vec<SystemDept> 转换为 Vec<DeptListData>
            //let dept_list_data_vec: Vec<DeptListData> = system_dept_vec.into_iter().map(|system_dept| DeptListData::from(system_dept)).collect();
            HttpResponse::Ok().json(ResVO::ok_with_data(system_dept_vec))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(
                "查询部门列表树异常,".to_string() + &err.to_string()
            ))
        }
    };
}





