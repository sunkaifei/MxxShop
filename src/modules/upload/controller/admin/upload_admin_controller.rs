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
use actix_web::{get, HttpResponse, web};
use crate::core::web::response::{ResultPage, ResVO};
use crate::modules::upload::entity::attach_model::{AttachPageRequest, AttachPageVO};

// 上传图片
pub async fn upload_product_image(form: MultipartForm<ImageForm>, ) -> HttpResponse {
    let form = form.into_inner();

    return attach_service::upload_product_image("product".to_string(), form).await;
}


#[get("/system/upload/list")]
pub async fn get_page_list(item: web::Query<AttachPageRequest>) -> HttpResponse {
    return match attach_service::get_page_list(item.0).await {
        Ok(page) => {
            let mut list_data: Vec<AttachPageVO> = Vec::new();
            for data in page.records {
                list_data.push(AttachPageVO {
                    id: data.id,
                    name: None,
                    path: None,
                    upload_url: None,
                    ext: None,
                    size: None,
                    md5: None,
                    r#type: None,
                    status: data.status,
                    add_time: data.add_time.map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
                })
            }
            let page_data = ResultPage {
                current_page: page.page_no,
                list: list_data,
                total: page.total,
            };
            HttpResponse::Ok().json(ResVO::ok_with_data(page_data))
        }
        Err(e) => {
            log::error!("查询附件出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}