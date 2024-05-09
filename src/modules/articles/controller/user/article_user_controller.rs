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
use actix_web::http::header::LOCATION;
use tera::Context;

use crate::core::service::templates_service;
use crate::core::web::response::ResVO;
use crate::modules::articles::entity::article_model::ArticlesSaveRequest;
use crate::modules::articles::service::article_service;
use crate::modules::user::entity::user_model::UserLoginSession;

pub async fn article_add(user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        let mut tera_ctx = Context::new();
        let user: UserLoginSession = serde_json::from_str(&user.id().unwrap_or_default()).unwrap();
        tera_ctx.insert("user", &user);
        let tera = templates_service::get_templates();
        let rendered = tera.render("default/article/add_article.html", &tera_ctx).unwrap_or_default();
        HttpResponse::Ok().body(rendered)
    } else {
        HttpResponse::TemporaryRedirect()
            .insert_header((LOCATION, "/user/login"))
            .finish()
    }
}

pub async fn article_post(user: Option<Identity>, item: web::Json<ArticlesSaveRequest>) -> HttpResponse {
    if let Some(user) = user {
        let mut article = item.0;
        let users: UserLoginSession = serde_json::from_str(&user.id().unwrap_or_default()).unwrap();
        article.user_id = users.id.clone();
        let result = article_service::save_article(article).await;
        HttpResponse::Ok().json(&ResVO::<u64>::handle_result(result))
    } else {
        HttpResponse::TemporaryRedirect()
            .insert_header((LOCATION, "/user/login"))
            .finish()
    }
}







