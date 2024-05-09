//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_web::web;

use crate::modules::articles::controller::user::{article_category_user_controller, article_user_controller};
use crate::modules::upload::controller::user::upload_user_controller;
use crate::modules::user::controller::user::{user_controller};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/login").route(web::get().to(user_controller::user_login)))
        .service(web::resource("/user/login").route(web::post().to(user_controller::post_login)))
        .service(web::resource("/user/signup").route(web::get().to(user_controller::get_signup)))
        .service(web::resource("/user/signup").route(web::post().to(user_controller::post_signup)))
        .service(web::resource("/user/index").route(web::get().to(user_controller::user_index)))
        .service(web::resource("/user/forgot/").route(web::get().to(user_controller::user_forgot)))
        .service(web::resource("/user/logout").route(web::post().to(user_controller::logout)))
        .service(web::resource("/user/add_article").route(web::get().to(article_user_controller::article_add)))
        .service(web::resource("/user/add_article").route(web::post().to(article_user_controller::article_post)))
        .service(web::resource("/user/all_ategory_tree").route(web::get().to(article_category_user_controller::all_ategory_tree)))
        .service(web::resource("/user/article/upload").route(web::post().to(upload_user_controller::upload_image)))
        .service(web::resource("/user/user/avatar").route(web::post().to(upload_user_controller::upload_avatar)))
    ;
}