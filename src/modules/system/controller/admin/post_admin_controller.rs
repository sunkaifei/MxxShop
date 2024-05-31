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
use crate::core::web::response::{ok_result_page, ResultPage, ResVO};
use crate::modules::system::entity::post_model::{PostListVO, PostPageBO, PostPageRequest};
use crate::modules::system::service::post_service;

#[get("/system/post/list")]
pub async fn get_post_page(item: web::Query<PostPageRequest>) -> HttpResponse {
    let page_req : PostPageBO = item.0.into();
    let page_result = post_service::select_by_page(page_req).await;
    return match page_result {
        Ok(page) => {
            let mut list_data: Vec<PostListVO> = Vec::new();
            for data in page.records {
                let data = PostListVO {
                    id: data.id,
                    name: data.name,
                    enabled: data.enabled,
                    sort: data.sort,
                    create_time: data.create_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default(),
                };
                list_data.push(data);
            }
            let page_data = ResultPage {
                current_page: page.page_no,
                list: list_data,
                total: page.total,
            };
            return HttpResponse::Ok().json(ok_result_page(page_data));
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(
                "获取角色参数列表异常,".to_string() + &err.to_string()
            ))
        }
    }
}