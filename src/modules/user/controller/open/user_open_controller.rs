//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use tera::Context;
use crate::core::service::templates_service;
use crate::core::web::entity::common::QueryUrl;
use crate::modules::user::service::user_service;
use crate::modules::user::entity::user_model::{UserInfo, UserLoginSession};

pub async fn user_index(user: Option<Identity>,path: web::Path<QueryUrl>) -> HttpResponse {
    let p_short_url = &path.short_url;
    
    let mut tera_ctx = Context::new();
    let tera = templates_service::get_templates();
    if let Some(user) = user {
        let user: UserLoginSession = serde_json::from_str(&user.id().unwrap_or_default()).unwrap();
        tera_ctx.insert("user", &user);
    }

    match user_service::get_by_short_url(&p_short_url).await {
        Ok(data_op) => {
            match data_op {
                Some(user_data) => {
                    let user_info:UserInfo = user_data.into();
                    tera_ctx.insert("userInfo", &user_info);
                }
                None => {
                    let rendered = tera.render("default/error/404.html", &tera_ctx).unwrap_or_default();
                    return HttpResponse::Ok().body(rendered);
                }
            }
        }
        Err(_) => {
            let rendered = tera.render("default/error/500.html", &tera_ctx).unwrap_or_default();
            return HttpResponse::Ok().body(rendered);
        }
    }

    let rendered = tera.render("default/user/index.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}
