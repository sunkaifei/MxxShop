//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_web::{HttpResponse};
use tera::Context;
use crate::core::service::templates_service;

pub async fn service_privacy() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.await.render("default/service/privacy.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}


pub async fn service_faq() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.await.render("default/service/faq.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}

pub async fn service_contacts() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.await.render("default/service/contacts.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}