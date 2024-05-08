use std::fs::File;
use std::io::Write;
use crate::modules::articles::entity::article_model::ArticleUser;
use crate::modules::user::service::user_service;
use actix_identity::Identity;
use actix_session::Session;

use actix_web::{HttpResponse};
use tera::Context;

use crate::core::service::templates_service;
use crate::modules::articles::entity::article_model::{ArticlesListData, ArticlesListRequest};
use crate::modules::articles::service::article_service;
use crate::modules::user::entity::user_model::{UserLoginSession};

pub async fn index(session: Session, user: Option<Identity>) -> HttpResponse {
    let session_captcha = session.get::<String>("captcha");
    match session_captcha {
        Ok(s_captcha) => {
            log::info!("====2=======s_captcha=======: {:?}",&s_captcha);
        }
        Err(_) => {}
    }
    let mut tera_ctx = Context::new();
    let select_list = ArticlesListRequest {
        category_id: None,
        user_id: None,
        status: None,
        page_no: 0,
        page_size: 10,
    };

    let mut result: Vec<ArticlesListData> = Vec::new();
    match article_service::select_list(select_list).await {
        Ok(list) => {

            let id_list: Vec<u64> = list.iter().map(|data| data.user_id).collect();
            let user_list_result = user_service::get_user_by_in_id(id_list).await;

            result = list.iter().map(|article| {
                let mut user_data = ArticleUser::default();

                match &user_list_result {
                    Ok(user_list) => {
                        for user in user_list {
                            if article.user_id.eq(&user.id) {
                                user_data = user.clone().into();
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        log::info!("========未获取用户信息====: ");
                    }
                };

                // 这里可以根据需要进行字段的映射、转换等操作
                ArticlesListData {
                    id: article.id.clone(),
                    short_url: article.short_url.clone(),
                    category_id: article.category_id.clone(),
                    category_ids: article.category_ids.clone(),
                    user: user_data,
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
        }
        Err(_) => {
            log::info!("========获取文章失败====: ");
        }
    };
    let tera = templates_service::get_templates();

    tera_ctx.insert("articleList", &result);


    if let Some(user) = user {
        let user: UserLoginSession = serde_json::from_str(&user.id().unwrap_or_default()).unwrap();
        tera_ctx.insert("user", &user);
    }
    let rendered = tera.render("default/index/index.html", &tera_ctx).unwrap_or_default();

    // 将渲染后的 HTML 内容写入静态文件
    let mut file = File::create("static/index.html").unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
    
    HttpResponse::Ok().body(rendered)
}


