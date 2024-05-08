use actix_files::Files as ActixFiles;
use actix_web::web;

use crate::modules::articles::controller::open::article_open_controller;
use crate::modules::statistics::controller::open::statistics_open_controller;
use crate::modules::system::controller::open::{captcha_controller, index_controller, service_controller};
use crate::modules::user::controller::open::user_open_controller;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(ActixFiles::new("/static/", "static/").prefer_utf8(true))
        .service(ActixFiles::new("/upload/", "storage/upload/").prefer_utf8(true))
        .service(web::resource("/").route(web::get().to(index_controller::index))) //首页
        .service(web::resource("/service/privacy").route(web::get().to(service_controller::service_privacy)))
        .service(web::resource("/service/faq").route(web::get().to(service_controller::service_faq)))
        .service(web::resource("/service/contacts").route(web::get().to(service_controller::service_contacts)))
        .service(web::resource("/a-list/{category}").route(web::get().to(article_open_controller::article_list)))
        .service(web::resource("/a-list/{category}/{page}").route(web::get().to(article_open_controller::article_list)))
        .service(web::resource("/a/{short_url}").route(web::get().to(article_open_controller::article_detail)))
        .service(web::resource("/u/{short_url}/").route(web::get().to(user_open_controller::user_index)))
        .service(captcha_controller::get_captcha)
        .service(statistics_open_controller::save_statistics_record)
    ;
}