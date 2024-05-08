use bcrypt::{DEFAULT_COST, hash};
use crate::core::errors::error::Result;
use rbatis::plugin::{Page, PageRequest};

use crate::modules::system::entity::admin_entity::SystemAdmin;
use crate::modules::system::entity::admin_model::{AdminSaveRequest, UserListRequest, UserLoginRequest, UserUpdateRequest};
use crate::modules::system::mapper::admin_mapper;
use crate::RB;
use crate::utils::snowflake_id::generate_snowflake_id;

pub async fn save_admin(user: AdminSaveRequest) -> Result<u64> {
    let mut sys_user:SystemAdmin =  user.into();

    sys_user.id = Option::from(generate_snowflake_id());
    let hashed = hash("123456", DEFAULT_COST).unwrap_or_default();
    sys_user.password = Option::from(hashed);
    let result = SystemAdmin::insert(&RB.clone(), &sys_user).await;

    return Ok(result.unwrap_or_default().rows_affected);
}

///批量删除用户信息
pub async fn delete_in_column(ids: Vec<Option<String>>) -> Result<u64> {
    let result = SystemAdmin::delete_in_column(&RB.clone(), "id", &ids).await;
    return Ok(result.unwrap_or_default().rows_affected);
}

///更新管理员信息
pub async fn update_by_user(user: UserUpdateRequest) -> Result<u64> {
    let admin:SystemAdmin = user.into();
    let result = SystemAdmin::update_by_column(&RB.clone(), &admin, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

pub async fn update_by_password(user: &SystemAdmin) -> Result<u64> {
    let result = SystemAdmin::update_by_column(&RB.clone(), &user, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

pub async fn select_by_id(id: &Option<u64>) -> rbatis::Result<Option<SystemAdmin>> {
    let result = SystemAdmin::select_by_column(&RB.clone(), "id", id).await?
        .into_iter()
        .next();
    return Ok(result);
}

pub async fn select_by_username(item: &UserLoginRequest) -> rbatis::Result<Option<SystemAdmin>> {
    let result = SystemAdmin::select_by_username(&RB.clone(), item.username.clone()).await;
    return result;
}

pub async fn select_user_page(item: UserListRequest) -> rbatis::Result<Page<SystemAdmin>> {
    let page_req = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let result = admin_mapper::select_user_page(&RB.clone(), page_req, &item).await;
    Ok(result?)
}
