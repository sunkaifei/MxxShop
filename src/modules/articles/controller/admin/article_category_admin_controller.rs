use actix_web::{get, HttpResponse, web};
use actix_web_grants::protect;
use crate::core::web::response::{ResVO};
use crate::modules::articles::entity::article_category_model::CategoryPageRequest;
use crate::modules::articles::service::article_category_service;

#[get("/system/category/list")]
#[protect("category:list:show")]
pub async fn category_list_tree(item: web::Query<CategoryPageRequest>) -> HttpResponse {
    let payload = item.0;
    return match article_category_service::select_all_list(payload).await{
        Ok(router_list) => {
            HttpResponse::Ok().json(&ResVO::ok_with_data(router_list))
        }
        Err(_err) => {
            HttpResponse::Ok().json(&ResVO::<String>::error_msg("未获取到文章分类列表".to_string()))
        }
    };
}