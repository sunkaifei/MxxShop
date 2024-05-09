//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::{HttpResponse};
use crate::core::web::response::ResVO;

use crate::modules::articles::service::article_category_service;


pub async fn all_ategory_tree(user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        // 菜单是树形结构不需要分页
        let result = article_category_service::all_ategory_tree().await;

        match result {
            Ok(router_list) => {
                HttpResponse::Ok().json(ResVO::ok_with_data(router_list))
            }
            Err(_err) => {
                HttpResponse::Ok().json(ResVO::<String>::error_msg("未获取到分类列表".to_string()))
            }
        }
    } else {
        HttpResponse::TemporaryRedirect()
            .insert_header((LOCATION, "/user/login"))
            .finish()
    }
}