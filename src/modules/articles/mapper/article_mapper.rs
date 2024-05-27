//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{crud, impl_select, impl_select_page, sql};
use rbatis::RBatis;

use crate::modules::articles::entity::article_entity::Articles;
use crate::modules::articles::entity::article_model::ArticlesListRequest;
use crate::modules::articles::entity::article_model::ArticlesPageRequest;

crud!(Articles {}, "mxx_article");

///根据文章标题查询是否存在
/// title 文章标题
/// user_id 用户ID
#[sql("select count(*) from mxx_article where title = ? and user_id = ?")]
pub async fn find_by_title_unique(
    rb: &RBatis,
    title: &Option<String>,
    user_id: &Option<u64>,
) -> rbatis::Result<u64> {
    impled!()
}

///查询短网址是否存在
#[sql("select count(*) from mxx_article where short_url = ?")]
pub async fn find_by_short_url_unique(
    rb: &RBatis,
    short_url: &str,
) -> rbatis::Result<u64> {
    impled!()
}


impl_select!(Articles{select_list(dto: &ArticlesListRequest) -> Vec =>"
     ` where 1=1`
     if dto.category_id != null && dto.category_id != '':
       ` and category_id = #{dto.category_id}`
     if dto.user_id != null && dto.user_id != '':
       ` and user_id = #{dto.user_id} `
     if dto.status != null && dto.status != '':
       ` and status = #{dto.status} `
     ` order by id desc `
     limit #{dto.page_size}
     "},"mxx_article");

impl_select_page!(Articles{select_page(dto: &ArticlesPageRequest) =>"
     trim end=' where ':
       ` where `
       trim ' and ':
         if dto.category_id != null && dto.category_id != '':
           ` and category_id = #{dto.category_id}`
         if dto.user_id != null && dto.user_id != '':
           ` and user_id = #{dto.user_id} `
         choose:
           when dto.status == 0:
             ` and status >= 0 `
           when dto.status == 1:
             ` and status = 0 `
           when dto.status == 2:
             ` and status = 1 `
     if !sql.contains('count'):
        ` order by id desc `"},"mxx_article");