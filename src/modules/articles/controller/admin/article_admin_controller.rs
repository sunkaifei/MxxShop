//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_web::{get, HttpResponse, web};
use crate::core::web::response::{err_result_page, ok_result_page, ResultPage};
use crate::modules::articles::entity::article_model::{ArticlesPageRequest,ArticlesPageData};
use crate::modules::articles::service::article_service;

#[get("/system/article/list")]
pub async fn get_article_list(item: web::Query<ArticlesPageRequest>) -> HttpResponse {
    let payload = item.0;
    let result = article_service::get_by_page(payload).await;

    match result {
        Ok(page) => {
            let mut list_data: Vec<ArticlesPageData> = Vec::new();
            for data in page.records {
                list_data.push(ArticlesPageData {
                    id: data.id,
                    category_id: data.category_id,
                    category_ids: data.category_ids,
                    user_id: data.user_id,
                    title: data.title,
                    title_image: data.title_image,
                    count_view: data.count_view,
                    isposts: data.isposts,
                    isrecommend: data.isrecommend,
                    //创建时间
                    create_time: data.create_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_else(|| "".to_string()),
                    status: data.status,
                })
            }
            let page_data = ResultPage {
                current_page: page.page_no,
                list: list_data,
                total: page.total,
            };

            HttpResponse::Ok().json(ok_result_page(page_data))
        }
        Err(err) => {
            HttpResponse::Ok().json(err_result_page(err.to_string()))
        }
    }
}