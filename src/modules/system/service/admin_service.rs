//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use bcrypt::{DEFAULT_COST, hash};
use crate::core::errors::error::Result;
use rbatis::plugin::{Page, PageRequest};

use crate::modules::system::entity::admin_entity::SystemAdmin;
use crate::modules::system::entity::admin_model::{AdminSaveRequest, UpdateAdminStatusRequest, UserListDTO, UserListRequest};
use crate::modules::system::mapper::admin_mapper;
use crate::modules::system::service::{depts_service, post_service, role_service};
use crate::pool;

pub async fn save_admin(admin: AdminSaveRequest) -> Result<u64> {
    let mut sys_admin:SystemAdmin =  admin.clone().into();
    let hashed = hash(sys_admin.password.unwrap_or_default(), DEFAULT_COST).unwrap_or_default();
    sys_admin.password = Option::from(hashed);
    let result = SystemAdmin::insert(pool!(), &sys_admin).await?;
    if result.rows_affected > 0{
        depts_service::batch_update_dept(&admin.dept_ids.clone(), &result.last_insert_id.as_u64()).await.unwrap_or_default();
        post_service::batch_update_post(&admin.post_ids.clone(), &result.last_insert_id.as_u64()).await.unwrap_or_default();
        role_service::batch_update_role(&admin.role_ids.clone(), &result.last_insert_id.as_u64()).await.unwrap_or_default();
    }
    return Ok(result.rows_affected);
}

///批量删除用户信息
pub async fn delete_in_column(ids: &Vec<Option<String>>) -> Result<u64> {
    
    let result = SystemAdmin::delete_in_column(pool!(), "id", ids).await;
    return Ok(result.unwrap_or_default().rows_affected);
}

///更新管理员信息
pub async fn update_admin(admin: SystemAdmin) -> Result<u64> {
    let result = SystemAdmin::update_by_column(pool!(), &admin, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

pub async fn update_by_password(user: &SystemAdmin) -> Result<u64> {
    let result = SystemAdmin::update_by_column(pool!(), &user, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

/// 更新用户状态
pub async fn update_by_status(user: UpdateAdminStatusRequest) -> Result<u64> {
    let admin:SystemAdmin = user.into();
    let result = SystemAdmin::update_by_column(pool!(), &admin, "id").await?;
    return Ok(result.rows_affected);
}

/// 根据名称查询用户是否唯一
pub async fn find_by_name_unique(user_name: &Option<String>, id: &Option<u64>) -> Result<bool> {
    let result = admin_mapper::find_by_name_unique(pool!(), user_name, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// 查询手机号是否是唯一
pub async fn find_by_mobile_unique(mobile: &Option<String>, id: &Option<u64>) -> Result<bool> {
    let result = admin_mapper::find_by_mobile_unique(pool!(), mobile, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// 查询邮箱是否是唯一
pub async fn find_by_email_unique(email: &Option<String>, id: &Option<u64>) -> Result<bool> {
    let result = admin_mapper::find_by_email_unique(pool!(), email, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// 查询昵称是否是唯一
pub async fn find_by_nick_name_unique(nick_name: &Option<String>, id: &Option<u64>) -> Result<bool> {
    let result = admin_mapper::find_by_nick_name_unique(pool!(), nick_name, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn get_by_detail(id: &Option<u64>) -> rbatis::Result<Option<SystemAdmin>> {
    let result = SystemAdmin::select_by_column(pool!(), "id", id).await?
        .into_iter()
        .next();
    return Ok(result);
}

///查询用户名
pub async fn select_by_username(username: &Option<String>) -> rbatis::Result<Option<SystemAdmin>> {
    let result = SystemAdmin::select_by_column(pool!(), "user_name",username).await?
    .into_iter()
        .next();
    return Ok(result);
}

pub async fn select_by_page(item: UserListRequest) -> rbatis::Result<Page<SystemAdmin>> {
    let item: UserListDTO = item.into();
    let page_req = &PageRequest::new(item.page_num.clone(), item.page_size.clone());
    let result = admin_mapper::select_by_page(pool!(), page_req, &item).await;
    Ok(result?)
}
