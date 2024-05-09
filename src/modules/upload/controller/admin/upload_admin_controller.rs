//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use crate::modules::upload::entity::attach_entity::ImageForm;
use crate::modules::upload::service::attach_service;
use actix_multipart::form::MultipartForm;
use actix_web::HttpResponse;

// 上传图片
pub async fn upload_product_image(form: MultipartForm<ImageForm>, ) -> HttpResponse {
    let form = form.into_inner();

    return attach_service::upload_product_image("product".to_string(), form).await;
}

