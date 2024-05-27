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
use crate::modules::product::entity::product_model::{ProductListVO, ProductPageDTO, ProductPageRequest};
use crate::modules::product::service::product_service;

#[get("/System/product/list")]
pub async fn get_product_list(item: web::Query<ProductPageRequest>) -> HttpResponse {
    let page_req : ProductPageDTO = item.0.into();
    let page_result = product_service::select_by_page(page_req).await;
    return match page_result {
        Ok(page) => {
            let mut list_data: Vec<ProductListVO> = Vec::new();
            for data in page.records {
                list_data.push(ProductListVO {
                    id: data.id,
                    lang_id: data.lang_id,
                    image: data.image,
                    product_name: data.product_name,
                    cate_id: data.cate_id,
                    price: data.price,
                    vip_price: data.vip_price,
                    ot_price: data.ot_price,
                    sort: data.sort,
                    sales: data.sales,
                    stock: data.stock,
                    unit_name: data.unit_name,
                    is_show: data.is_show,
                    create_time: Option::from(data.create_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default()),
                });
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
                "获取系统参数列表异常,".to_string() + &err.to_string()
            ))
        }
    }
}