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
use crate::modules::system::entity::admin_model::{AdminSaveRequest, UserListDTO, UserListRequest, UserLoginRequest, UserUpdateRequest};
use crate::modules::system::mapper::admin_mapper;
use crate::pool;

pub async fn save_admin(user: AdminSaveRequest) -> Result<u64> {
    let mut sys_user:SystemAdmin =  user.into();

    let hashed = hash("123456", DEFAULT_COST).unwrap_or_default();
    sys_user.password = Option::from(hashed);
    let result = SystemAdmin::insert(pool!(), &sys_user).await;

    return Ok(result.unwrap_or_default().rows_affected);
}

///批量删除用户信息
pub async fn delete_in_column(ids: &Vec<Option<String>>) -> Result<u64> {
    
    let result = SystemAdmin::delete_in_column(pool!(), "id", ids).await;
    return Ok(result.unwrap_or_default().rows_affected);
}

///更新管理员信息
pub async fn update_by_user(user: UserUpdateRequest) -> Result<u64> {
    let admin:SystemAdmin = user.into();
    let result = SystemAdmin::update_by_column(pool!(), &admin, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

pub async fn update_by_password(user: &SystemAdmin) -> Result<u64> {
    let result = SystemAdmin::update_by_column(pool!(), &user, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

pub async fn select_by_id(id: &Option<u64>) -> rbatis::Result<Option<SystemAdmin>> {
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
