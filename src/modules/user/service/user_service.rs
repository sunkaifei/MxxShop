use actix_web::HttpResponse;
use crate::core::errors::error::Result;
use crate::core::web::response::ResVO;

use crate::modules::user::entity::user_entity::User;
use crate::modules::user::entity::user_model::{UserLoginRequest, UserSaveDTO, UserSaveRequest};
use crate::modules::user::mapper::user_mapper;
use crate::RB;
use crate::utils::snowflake_id;

///用户注册
pub async fn add_user(payload: UserSaveRequest) -> Result<u64> {
    let mut user_data = UserSaveDTO {
        id: 0,
        user_name: payload.user_name,
        email: None,
        mobile: None,
        password: "".to_string(),
        salt: None,
        reg_time: None,
        reg_ip: None,
        last_login: None,
        last_ip: None,
        skin: None,
        is_del: 0,
    };
    match snowflake_id::generate_unique_id() {
        Ok(id) => {
            user_data.id = id;
        }
        Err(err) => eprintln!("Error: {}", err),
    }
    let mut tx = RB.acquire_begin().await?;
    let rows = user_mapper::add_user(&RB.clone(), &user_data).await;
    tx.commit().await?;
    tx.rollback().await?;
    return Ok(rows.unwrap_or_default().rows_affected);
}

///批量删除用户
pub async fn delete_in_column(ids: Vec<String>) -> HttpResponse {
    let result = User::delete_in_column(&RB.clone(), "id", &ids).await;
    HttpResponse::Ok().json(ResVO::<u64>::handle_result(Ok(result.unwrap_or_default().rows_affected)))
}


pub async fn select_by_username(item: &UserLoginRequest) -> rbatis::Result<Option<User>> {
    let result = User::select_by_username(&RB.clone(), &item.username).await;
    return result;
}

///按短网址查询用户信息
pub async fn get_by_short_url(short_url: &Option<String>) -> rbatis::Result<Option<User>> {
    let st = User::select_by_column(&RB.clone(), "short_url", short_url).await?
        .into_iter().next();
    Ok(st)
}

///in姓氏id和名字id查出名字
pub async fn get_user_by_in_id(
    user_ids: Vec<u64>,
) -> rbatis::Result<Vec<User>> {
    let result = user_mapper::get_user_by_in_id(&RB.clone(), user_ids).await;
    return result;
}