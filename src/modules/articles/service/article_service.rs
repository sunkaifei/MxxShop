//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use jieba_rs::Jieba;
use rbatis::plugin::{Page, PageRequest};

use crate::core::errors::error::Error;
use crate::core::errors::error::Result;
use crate::modules::articles::entity::article_entity::Articles;
use crate::modules::articles::entity::article_model::{ArticlesListRequest, ArticlesPageRequest, ArticlesSaveRequest, ArticlesUpdateRequest};
use crate::modules::articles::mapper::article_mapper;
use crate::pool;
use crate::utils::{short_url};
use crate::utils::snowflake_id::generate_snowflake_id;

///添加文章
pub async fn save_article(payload: ArticlesSaveRequest) -> Result<u64> {
    let mut article_data: Articles = payload.into();
    article_data.id = Some(generate_snowflake_id());
    //log::info!("========article_data====: {:?}", &article_data.user_id);
    let unique_num = article_mapper::find_by_title_unique(pool!(), &article_data.title, &article_data.user_id).await?;
    if unique_num > 0 {
        return Err(Error::from("文章标题已存在".to_string()));
    }

    //获取短网址唯一性
    article_data.short_url = find_short_url_unique().await;


    let rows = Articles::insert(pool!(), &article_data).await;
    if rows.is_ok() {
        let jieba = Jieba::new();
        let title = article_data.title.clone().unwrap_or_default();
        let words = jieba.cut(&title, false);

        for word in words {
            println!("{}", word);
        }
    }
    return Ok(rows.unwrap_or_default().rows_affected);
}

///获取短网址唯一性
pub async fn find_short_url_unique() -> Option<String> {

    let mut new_short_url: Option<String> = Some(String::new());
    for _ in 0..5 {
        //获取短网址
        let short_url = short_url::generate_random_code(5).unwrap_or_default();
        let unique_num = article_mapper::find_by_short_url_unique(pool!(), &short_url).await;
        if unique_num.unwrap_or(0) == 0 {
            new_short_url = Some(short_url);
            break;
        }
    }
    return new_short_url;
}


///按id删除文章
pub async fn delete_article(id: u64) -> Result<u64> {
    let rows = Articles::delete_by_column(pool!(), "id", id).await;
    return Ok(rows.unwrap_or_default().rows_affected);
}

///按id数组批量删除文章
pub async fn delete_article_by_ids(ids: Vec<String>) -> Result<u64> {
    let rows = Articles::delete_by_column(pool!(), "id", ids).await;
    return Ok(rows.unwrap_or_default().rows_affected);
}

///更新文章内容
pub async fn update_article(payload: ArticlesUpdateRequest) -> Result<u64> {
    let article_data: Articles = payload.into();
    let rows = Articles::update_by_column(pool!(), &article_data, "id").await;
    return Ok(rows.unwrap_or_default().rows_affected);
}


pub async fn get_by_id(id: u64) -> rbatis::Result<Option<Articles>> {
    let st = Articles::select_by_column(pool!(), "id", id).await?
        .into_iter().next();
    Ok(st)
}

pub async fn get_by_short_url(short_url: Option<String>) -> rbatis::Result<Option<Articles>> {
    let st = Articles::select_by_column(pool!(), "short_url", short_url).await?
        .into_iter().next();
    Ok(st)
}

///查询文章列表
pub async fn select_list(mut item: ArticlesListRequest) -> rbatis::Result<Vec<Articles>> {
    let page_req = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    item.page_no = page_req.page_no;
    item.page_size = page_req.page_size;
    let list = Articles::select_list(pool!(), &item).await;
    return list;
}

pub async fn get_by_page(item: ArticlesPageRequest) -> rbatis::Result<Page<Articles>> {
    let page_req = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let list = Articles::select_page(pool!(), page_req, &item).await;
    return list;
}


#[cfg(test)]
mod tests {
    use jieba_rs::Jieba;

    #[test]
    fn test_addition() {
        let jieba = Jieba::new();
        let words = jieba.cut_for_search("我爱自然语言处理", false);
        for word in words {
            println!("{}", word);
        }
    }
}