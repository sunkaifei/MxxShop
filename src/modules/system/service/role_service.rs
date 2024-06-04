//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_web::HttpResponse;
use rbatis::rbdc::datetime::DateTime;
use rbatis::plugin::{Page, PageRequest};
use crate::core::errors::error::{Error, Result};
use crate::core::web::response::ResVO;
use crate::modules::system::entity::admin_role_entity::SystemAdminRole;
use crate::modules::system::entity::admin_role_model::UpdateUserRoleRequest;
use crate::modules::system::entity::role_entity::SystemRole;
use crate::modules::system::entity::role_menu_entity::SystemRoleMenu;
use crate::modules::system::entity::role_model::{RoleDTO};
use crate::modules::system::mapper::role_mapper;
use crate::pool;
use crate::utils::snowflake_id::{generate_snowflake_id};

pub async fn save_role(payload: &RoleDTO) -> Result<u64> {
    let unique_num = role_mapper::find_role_by_name_unique(pool!(), &payload.role_name, &None).await.unwrap_or_default();
    if unique_num > 0 {
        return Err(Error::from("该角色已存在!".to_string()));
    }
    // 生成id
    let mut role_entity = SystemRole::default();
    role_entity.id = Option::from(generate_snowflake_id());
    role_entity.process_role_dto(&payload);
    role_entity.create_time = Option::from(DateTime::now());
    role_entity.update_time = Option::from(DateTime::now());
    let result = SystemRole::insert(pool!(), &role_entity).await?;

    let rows = result.rows_affected;
    let last_insert_id = result.last_insert_id.as_u64();
    return if rows > 0 {
        if payload.menu_ids.clone().is_some_and(|vec| !vec.is_empty() && !vec.iter().all(|item| item.is_none())) {
            if let Some(menu_ids) = payload.menu_ids.clone() {
                // 使用 filter 方法筛选不为空且不为 None 的元素
                let filtered_ids: Vec<Option<String>> = menu_ids
                    .into_iter()
                    .filter_map(|item| {
                        item.and_then(|s| {
                            if s.trim().is_empty() {
                                None
                            } else {
                                Some(s)
                            }
                        })
                    })
                    .map(Some)
                    .collect();

                // 检查筛选后的数组是否不为空
                if !filtered_ids.is_empty() {
                    let menu_ids = filtered_ids
                            .into_iter()
                            .map(|opt_str| opt_str.map(|s| s.parse::<u64>().ok()).flatten()) // 使用 flatten 方法展平 Option<Option<u64>>
                            .collect();
                    insert_batch(menu_ids, &last_insert_id).await?;
                }
            }
        }
        Ok(rows)
    } else {
        Ok(0)
    }
}

pub async fn insert_batch(menu_ids: Vec<Option<u64>>, role_id: &Option<u64>) -> Result<u64> {
    let role_menu_result = SystemRoleMenu::delete_by_column(pool!(), "role_id", &role_id).await;
    return match role_menu_result {
        Ok(_) => {
            let mut menu_role: Vec<SystemRoleMenu> = Vec::new();

            for menu_id in &menu_ids {
                menu_role.push(SystemRoleMenu {
                    id: Option::from(generate_snowflake_id()),
                    menu_id: menu_id.clone(),
                    role_id: role_id.clone(),
                    create_time: Some(DateTime::now()),
                    update_time: Some(DateTime::now()),
                    status: Option::from(0),
                })
            }

            let result = SystemRoleMenu::insert_batch(pool!(), &menu_role, menu_ids.len() as u64).await;

            Ok(result?.rows_affected)
        }
        Err(err) => {
            log::error!("批量插入角色和菜单关联失败，insert_batch: {:?}", err);
            Ok(0)
        }
    }
}

///按id批量删除角色信息
pub async fn delete_in_column(ids_vec: Vec<Option<String>>) -> Result<u64> {
    return if ids_vec.is_empty() {
        Err(Error::from("删除的ID不能为空".to_string()))
    } else {
        let user_role_list = SystemAdminRole::select_in_column(pool!(), "role_id", &ids_vec).await.unwrap_or_default();
        if user_role_list.len() > 0 {
            return Err(Error::from("删除的ID不能为空".to_string()));
        }
        let result = SystemRole::delete_in_column(pool!(), "id", &ids_vec).await?;
        if result.rows_affected > 0 {
            SystemRoleMenu::delete_in_column(pool!(), "role_id", &ids_vec).await?;
        }
        Ok(result.rows_affected)
    }
}

///按id批量删除角色和菜单关联信息
pub async fn delete_role_menu_column(id: &u64) -> Result<u64> {
    let result = SystemRoleMenu::delete_by_column(pool!(), "role_id", &id).await;
    return Ok(result.unwrap_or_default().rows_affected);
}

pub async fn update_user_role(item: UpdateUserRoleRequest) -> HttpResponse {
    let admin_id = item.admin_id;
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    if admin_id == 1 {
        HttpResponse::Ok().json(ResVO::<String>::error_msg("不能修改超级管理员的角色".to_string()));
    }

    let sys_result = SystemAdminRole::delete_by_column(pool!(), "user_id", admin_id).await;

    if sys_result.is_err() {
        HttpResponse::Ok().json(ResVO::<String>::error_msg("更新用户角色异常".to_string()));
    }

    let mut sys_role_user_list: Vec<SystemAdminRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(SystemAdminRole {
            create_time: Some(DateTime::now()),
            role_id: r_id,
            admin_id: admin_id.clone(),
        })
    }

    let result = SystemAdminRole::insert_batch(pool!(), &sys_role_user_list, len as u64).await;

    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(Ok(result.unwrap_or_default().rows_affected)));
}

pub async fn update_role(payload: &RoleDTO) -> Result<u64> {
    let mut role_entity = SystemRole::default();
    role_entity.process_role_dto(&payload);
    role_entity.update_by = payload.admin.user_name.clone();
    role_entity.update_time = Option::from(DateTime::now());
    let result = SystemRole::update_by_column(pool!(), &role_entity, "id").await?;
    let rows = result.rows_affected;
    return if rows > 0 {
        if payload.menu_ids.clone().is_some_and(|vec| !vec.is_empty() && !vec.iter().all(|item| item.is_none())) {
            if let Some(menu_ids) = payload.menu_ids.clone() {
                // 使用 filter 方法筛选不为空且不为 None 的元素
                let filtered_ids: Vec<Option<String>> = menu_ids
                    .into_iter()
                    .filter_map(|item| {
                        item.and_then(|s| {
                            if s.trim().is_empty() {
                                None
                            } else {
                                Some(s)
                            }
                        })
                    })
                    .map(Some)
                    .collect();

                // 检查筛选后的数组是否不为空
                if !filtered_ids.is_empty() {
                    let menu_ids = filtered_ids
                        .into_iter()
                        .map(|opt_str| opt_str.map(|s| s.parse::<u64>().ok()).flatten()) // 使用 flatten 方法展平 Option<Option<u64>>
                        .collect();
                    insert_batch(menu_ids, &role_entity.id.clone()).await?;
                }
            }
        }else{
            SystemRoleMenu::delete_by_column(pool!(), "role_id", &role_entity.id).await?;
        }
        Ok(rows)
    } else {
        Ok(0)
    }
}

/// 查询角色名称是否已存在
pub async fn find_role_by_name_unique(role_name: Option<String>, id: Option<u64>) -> Result<u64> {
    return Ok(role_mapper::find_role_by_name_unique(pool!(), &role_name, &id).await.unwrap_or_default());
}

///查询角色详情
pub async fn get_by_detail(id: &Option<u64>) -> rbatis::Result<Option<SystemRole>> {
    Ok(SystemRole::select_by_column(pool!(),"id", id).await?
        .into_iter()
        .next())
}

///角色id查询所有关联的菜单id
pub async fn get_merge_by_role_id(role_id: &Option<u64>) -> Result<Vec<Option<String>>> {
    let result = role_mapper::get_merge_by_role_id(pool!(), role_id).await?;
    let ids: Vec<Option<String>> = result.iter().map(|merge| merge.menu_id.map(|menu_id| menu_id.to_string())).collect();
    return Ok(ids);
}

///查询角色是否已被关联
pub async fn select_in_column(ids: &Vec<String>) -> rbatis::Result<Vec<SystemAdminRole>> {
    let result = SystemAdminRole::select_in_column(pool!(), "role_id", &ids).await;
    return result;
}

///查询用户关联的角色列表
pub async fn query_admin_by_role(admin_id: u64) -> rbatis::Result<Vec<SystemRole>> {
    let result = role_mapper::query_admin_by_role(pool!(), admin_id).await;
    return result;
}

///查询当前菜单id关联的所有角色ID
pub async fn query_menu_by_role(menu_id : Option<u64>) -> Vec<u64> {
    let result = role_mapper::query_menu_by_role(pool!(), menu_id).await;
    let mut vec_of_u64: Vec<u64> = Vec::new();
    match result {
        Ok(v) => {
            for hashmap in v {
                for (_, value) in hashmap {
                    vec_of_u64.push(value);
                }
            }
        },
        Err(err) => {
            log::error!("当前菜单id关联的所有角色ID出错: {:?}", &err);
        }
    }

    return vec_of_u64;
}

///查询所有角色列表
pub async fn select_all() -> rbatis::Result<Vec<SystemRole>> {
    let result = SystemRole::select_all(pool!()).await;
    return result;
}

///查询用户权限关联的所有权限标识
pub async fn get_admin_by_role(admin_id: u64) -> rbatis::Result<Vec<String>> {
    let result = role_mapper::get_admin_by_role(pool!(), admin_id).await;
    return result;
}

///查询角色分页
pub async fn page(role_name: &str, status: &str, page_req: PageRequest, ) -> rbatis::Result<Page<SystemRole>> {
    let result = SystemRole::select_page(pool!(), &page_req, role_name, status).await;
    return result;
}
