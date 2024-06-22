//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_web::{delete, get, HttpResponse, post, put, web};
use crate::core::web::entity::common::{BathIdRequest, InfoId};
use crate::core::web::response::{ok_result_page, ResultPage, ResVO};
use crate::modules::system::entity::config_model::{ConfigPageBO, ConfigPageRequest, ConfigSaveRequest, ConfigUpdateRequest, ConfigVO, SystemConfigVO};
use crate::modules::system::service::{config_service};


#[post("/system/config/save")]
pub async fn save_config(item: web::Json<ConfigSaveRequest>) -> HttpResponse {
    if item.config_key.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息key不能为空".to_string()));
    }
    if item.config_name.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息名称不能为空".to_string()));
    }
    if config_service::find_by_key_unique(&item.config_key, &None).await.unwrap_or_default() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息key已存在".to_string()));
    }
    if config_service::find_by_name_unique(&item.config_name, &None).await.unwrap_or_default() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息name已存在".to_string()));
    }
    return match config_service::save_config(item.0).await {
        Ok(user_op) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(user_op))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

#[delete("/system/config/batch_delete")]
pub async fn batch_delete(item: web::Json<BathIdRequest>) -> HttpResponse {
    let ids = item.0;
    let result = config_service::batch_delete(ids).await;
    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
}

#[put("/system/config/update")]
pub async fn update_config(item: web::Json<ConfigUpdateRequest>) -> HttpResponse {
    if item.config_id.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息id不能为空".to_string()));
    };
    if item.config_key.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息key不能为空".to_string()));
    };
    if item.config_name.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息名称不能为空".to_string()));
    };
    if config_service::find_by_key_unique(&item.config_key, &None).await.unwrap_or_default() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息key已存在".to_string()));
    };
    if config_service::find_by_name_unique(&item.config_name, &None).await.unwrap_or_default() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息name已存在".to_string()));
    };
    return match config_service::update_config(item.0).await {
        Ok(user_op) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(user_op))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

#[get("/system/config/detail/{id}")]
pub async fn get_config_detail(item: web::Path<InfoId>) -> HttpResponse {
    if item.id.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("配置信息id不能为空".to_string()));
    }
    return match config_service::get_by_detail(&item.id).await {
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
    let page_result = config_service::select_by_page(page_req).await;
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