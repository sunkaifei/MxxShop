//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use regex::Regex;
use actix_web::Error;
use std::borrow::Cow;
use crate::core::web::entity::common::QueryUrl;
use actix_web::{HttpResponse, web};
use actix_web::web::Redirect;
use actix_identity::Identity;
use tera::Context;

use crate::core::service::templates_service;
use crate::core::web::entity::common::Pagination;
use crate::modules::user::entity::user_model::UserLoginSession;
use crate::modules::articles::entity::article_model::{ArticlesListData, ArticlesPageRequest, ArticlesDetailData, Info, ArticleUser};
use crate::modules::articles::service::article_service;

pub async fn article_list(info: web::Path<Info>) -> HttpResponse {
    //let (category, page) = info.into_inner();
    let param = ArticlesPageRequest {
        category_id: None,
        user_id: None,
        status: None,
        page_no:1,
        page_size: 10,
    };
    let mut tera_ctx = Context::new();

    let mut result: Vec<ArticlesListData> = Vec::new();
    match article_service::get_by_page(param).await {
        Ok(data_op) => {
            result = data_op.records.iter().map(|article| {
                // 这里可以根据需要进行字段的映射、转换等操作
                ArticlesListData {
                    id: article.id.clone(),
                    short_url: article.short_url.clone(),
                    category_id: article.category_id.clone(),
                    category_ids: article.category_ids.clone(),
                    user: ArticleUser::default(),
                    title: article.title.clone(),
                    short_title: article.short_title.clone(),
                    title_image: article.title_image.clone(),
                    author: article.author.clone(),
                    original_link: article.original_link.clone(),
                    description: article.description.clone(),
                    content: article.content.clone(),
                    count_comment: article.count_comment.clone(),
                    count_view: article.count_view.clone(),
                    count_love: article.count_love.clone(),
                    count_digg: article.count_digg.clone(),
                    count_burys: article.count_burys.clone(),
                    count_follow: article.count_follow.clone(),
                    isclose: article.isclose.clone(),
                    iscomment: article.iscomment.clone(),
                    iscommentshow: article.iscommentshow.clone(),
                    isposts: article.isposts.clone(),
                    isrecommend: article.isrecommend.clone(),
                    update_time: article.update_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
                    create_time: article.create_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
                }
            }).collect();

            let pagination = Pagination {
                current: data_op.page_no,
                total: data_op.total,
                page_size: data_op.page_size,
            };
            tera_ctx.insert("pagination", &pagination);
        }
        Err(_) => {}
    }
    let tera = templates_service::get_templates();

    tera_ctx.insert("articleList", &result);


    let rendered = tera.render("default/article/list_article.html", &tera_ctx).unwrap_or_default();
    HttpResponse::Ok().body(rendered)
}


///文章详细页面
pub async fn article_detail(user: Option<Identity>, path: web::Path<QueryUrl>) -> Result<HttpResponse, Error> {
    let s_short_url = path.short_url.clone();
    if s_short_url.is_some() {
        Redirect::to("/error/400").permanent();
    }
    let mut tera_ctx = Context::new();
    log::info!("=============short_url=====================: {:?}", &s_short_url);

    match article_service::get_by_short_url(s_short_url).await {
        Ok(data_op) => {
            match data_op {
                Some(data) => {
                    let mut articles_data = ArticlesDetailData {
                        id: data.id,
                        short_url: data.short_url,
                        category_id: None,
                        category_ids: None,
                        user_id: data.user_id,
                        title: data.title,
                        short_title: data.short_title,
                        title_image: data.title_image,
                        author: data.author,
                        original_link: data.original_link,
                        description: data.description,
                        content: data.content,
                        count_comment: data.count_comment,
                        count_view: data.count_view,
                        count_love: data.count_love,
                        count_digg: data.count_digg,
                        count_burys: data.count_burys,
                        count_follow: data.count_follow,
                        isclose: data.isclose,
                        iscomment: data.iscomment,
                        iscommentshow: data.iscommentshow,
                        isposts: data.isposts,
                        isrecommend: data.isrecommend,
                        update_time: data.update_time,
                    };

                    let re = Regex::new(r#"<[^>]*>"#).unwrap();
                    let stripped_content = if let Some(content) = &articles_data.content {
                        re.replace_all(content, "")
                    } else {
                        Cow::Borrowed("")
                    };


                    articles_data.description = Some(stripped_content.to_string());
                    tera_ctx.insert("description", &articles_data);
                    tera_ctx.insert("field", &articles_data);
                }
                None => {
                    Redirect::to("/error/400").permanent();
                }
            }
        }
        Err(_err) => {
            Redirect::to("/error/500").permanent();
        }
    };
    if let Some(user) = user {
        let user: UserLoginSession = serde_json::from_str(&user.id().unwrap_or_default()).unwrap();
        tera_ctx.insert("user", &user);
    }
    let tera = templates_service::get_templates();
    let rendered = tera.render("default/article/detail_article.html", &tera_ctx).unwrap_or_default();
    Ok(HttpResponse::Ok().body(rendered))
}


#[cfg(test)]
mod tests {
    use scraper::Html;

    #[test]
    fn test_article_detail() {
        let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

        let document = Html::parse_document(html);
        println!("{:?}", document);
    }
}
