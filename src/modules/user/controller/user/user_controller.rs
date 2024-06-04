//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_http::HttpMessage;
use actix_identity::Identity;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use bcrypt::verify;
use tera::Context;

use crate::core::service::templates_service;
use crate::core::web::response::ResVO;
use crate::modules::user::entity::user_model::{UserLoginRequest, UserLoginResponse, UserLoginSession, UserSaveRequest};
use crate::modules::user::service::user_service;

pub async fn user_login() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.await.render("default/user/login.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}

pub async fn post_login(request: HttpRequest, item: web::Json<UserLoginRequest>) -> HttpResponse {
    let user_result = user_service::select_by_username(&item).await;
    match user_result {
        Ok(user) => {
            match user {
                None => {
                    HttpResponse::Ok().json(ResVO::<String>::error_msg("用户不存在".to_string()))
                }
                Some(user_info) => {
                    let valid = verify(&item.password.clone().unwrap_or_default(), &user_info.password).unwrap_or_default();
                    if !valid {
                        return HttpResponse::Ok().json(ResVO::<String>::error_msg("密码不正确".to_string()));
                    }

                    let logger_user = UserLoginResponse {
                        id: user_info.id.clone(),
                        username: user_info.user_name.clone(),
                    };
                    let logger_string = serde_json::to_string(&logger_user).unwrap_or_default();
                    let _ = Identity::login(&request.extensions(), logger_string.into());
                    return HttpResponse::Ok().json(ResVO::<String>::error_msg("登录成功".to_string()));
                }
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

///用户注册
pub async fn get_signup() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.await.render("default/user/signup.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}

pub async fn post_signup(item: web::Json<UserSaveRequest>) -> HttpResponse {
    let user_save = item.0;
    let result = user_service::add_user(user_save).await;

    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
}

pub async fn user_index(user: Option<Identity>) -> HttpResponse {

    let mut tera_ctx = Context::new();
    if let Some(user) = user {
        let user: UserLoginSession = serde_json::from_str(&user.id().unwrap_or_default()).unwrap();
        tera_ctx.insert("user", &user);
    }

    // 取出用户名
    let tera = templates_service::get_templates();
    let rendered = tera.await.render("default/user/index.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}

pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    return HttpResponse::Ok().json(ResVO::<String>::error_msg("已退出".to_string()));
}


pub async fn user_forgot() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.await.render("default/user/forgot.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}