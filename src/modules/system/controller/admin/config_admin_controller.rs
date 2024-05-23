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
use crate::core::web::entity::common::InfoId;
use crate::core::web::response::{ok_result_page, ResultPage, ResVO};
use crate::modules::system::entity::config_model::{ConfigPageBO, ConfigPageRequest, ConfigVO, SystemConfigVO};
use crate::modules::system::service::{config_service};


#[get("/system/config/detail/{id}")]
pub async fn get_config_detail(item: web::Path<InfoId>) -> HttpResponse {
    if item.id.clone().is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息id不能为空".to_string()));
    }
    let string_id = item.into_inner().id.clone().unwrap_or_default();
    let u64_id: u64 = string_id.parse::<u64>().unwrap_or_else(|_| 0);
    return match config_service::get_config_by_id(Option::from(u64_id)).await {
        Ok(user_op) => match user_op {
            None => {
                HttpResponse::Ok().json(ResVO::<String>::error_msg("查询的配置信息不存在".to_string()))
            }
            Some(role) => {
                let config_detail: ConfigVO = role.into();
                HttpResponse::Ok().json(ResVO::ok_with_data(config_detail))
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

// 分页查询
#[get("/system/config/list")]
pub async fn get_config_page(item: web::Query<ConfigPageRequest>) -> HttpResponse {
    let page_req : ConfigPageBO = item.0.into();
    let page_result = config_service::select_config_page(page_req).await;
    return match page_result {
        Ok(page) => {
            let mut list_data: Vec<SystemConfigVO> = Vec::new();
            for data in page.records {
                list_data.push(SystemConfigVO {
                    config_id: data.config_id,
                    config_name: data.config_name,
                    config_key: data.config_key,
                    config_value: data.config_value,
                    config_type: data.config_type,
                    remark: data.remark,
                    sort: data.sort,
                    update_time: data.update_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default(),
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