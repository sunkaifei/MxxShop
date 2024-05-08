use actix_web::{HttpResponse};
use tera::Context;
use crate::core::service::templates_service;

pub async fn service_privacy() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.render("default/service/privacy.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}


pub async fn service_faq() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.render("default/service/faq.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}

pub async fn service_contacts() -> HttpResponse {
    let tera_ctx = Context::new();
    //tera_ctx .insert("field", &result);
    let tera = templates_service::get_templates();
    let rendered = tera.render("default/service/contacts.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}