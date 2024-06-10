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
use crate::modules::system::entity::post_model::{PostListVO, PostPageBO, PostPageRequest, PostSaveRequest, PostUpdateRequest};
use crate::modules::system::service::{post_service};

#[post("/system/post/save")]
pub async fn save_post(item: web::Json<PostSaveRequest>) -> HttpResponse {
    if item.post_name.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("岗位名称不能为空".to_string()));
    }
    if post_service::find_by_name(&item.post_name, &None).await.unwrap_or_default() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("岗位名称已存在".to_string()));
    }
    let result = post_service::save_post(item.0).await;
    return match result {
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::<u64>::ok_with_data(v))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

#[delete("/system/post/bath_delete")]
pub async fn bath_delete_post(item: web::Json<BathIdRequest>) -> HttpResponse {
    if let Some(ids_vec) = item.ids.clone() {
        if ids_vec.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()));
        } else {
            let result = post_service::delete_in_column(ids_vec).await;
            return match result {
                Ok(v) => {
                    HttpResponse::Ok().json(ResVO::<u64>::ok_with_data(v))
                }
                Err(err) => {
                    HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
                }
            }
        }
    }else {
        HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()))
    }
}

#[put("/system/post/update")]
pub async fn update_post(item: web::Json<PostUpdateRequest>) -> HttpResponse {
    if item.post_name.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("岗位名称不能为空".to_string()));
    }
    if post_service::find_by_name(&item.post_name, &item.id).await.unwrap_or_default() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("岗位名称已存在".to_string()));
    }
    let result = post_service::update_post(item.0).await;
    return match result {
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::<u64>::ok_with_data(v))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

#[get("/system/post/detail/{id}")]
pub async fn get_post_detail(item: web::Path<InfoId>) -> HttpResponse {
    if item.id.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("ID不能为空".to_string()));
    }
    return match post_service::get_by_detail(&item.id).await {
        Ok(role_op) => match role_op {
            None => {
                HttpResponse::Ok().json(ResVO::<String>::error_msg("角色信息不存在".to_string()))
            }
            Some(post) => {
                let post_vo = PostListVO{
                    id: post.id,
                    post_code: post.post_code,
                    post_name: post.post_name,
                    enabled: post.enabled,
                    sort: post.sort,
                    remark: post.remark,
                    create_time: post.create_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default(),
                };
                HttpResponse::Ok().json(ResVO::ok_with_data(post_vo))
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

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
                    post_code: data.post_code,
                    post_name: data.post_name,
                    enabled: data.enabled,
                    sort: data.sort,
                    remark: data.remark,
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