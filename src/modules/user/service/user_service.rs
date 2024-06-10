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
use crate::core::errors::error::Result;
use crate::core::web::response::ResVO;

use crate::modules::user::entity::user_entity::User;
use crate::modules::user::entity::user_model::{UserLoginRequest, UserSaveRequest};
use crate::modules::user::mapper::user_mapper;
use crate::pool;

///用户注册
pub async fn add_user(user: UserSaveRequest) -> Result<u64> {
    let mut user_data = user.into();
    let rows = User::insert(pool!(), &user_data).await;
    return Ok(rows.unwrap_or_default().rows_affected);
}

///批量删除用户
pub async fn delete_in_column(ids: Vec<String>) -> HttpResponse {
    let result = User::delete_in_column(pool!(), "id", &ids).await;
    HttpResponse::Ok().json(ResVO::<u64>::handle_result(Ok(result.unwrap_or_default().rows_affected)))
}


pub async fn select_by_username(item: &UserLoginRequest) -> rbatis::Result<Option<User>> {
    let result = User::select_by_username(pool!(), &item.username).await;
    return result;
}

///按短网址查询用户信息
pub async fn get_by_short_url(short_url: &Option<String>) -> rbatis::Result<Option<User>> {
    let st = User::select_by_column(pool!(), "short_url", short_url).await?
        .into_iter().next();
    Ok(st)
}

///in姓氏id和名字id查出名字
pub async fn get_user_by_in_id(
    user_ids: Vec<u64>,
) -> rbatis::Result<Vec<User>> {
    let result = user_mapper::get_user_by_in_id(pool!(), user_ids).await;
    return result;
}