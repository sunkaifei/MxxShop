//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use rbatis::{crud, sql};
use rbatis::RBatis;

use crate::modules::articles::entity::article_category_entity::ArticleCategory;

crud!(ArticleCategory {}, "fly_article_category");

///查询短网址是否存在
#[sql("select count(*) from fly_article_category where category_name = ?")]
pub async fn find_by_category_name_unique(
    rb: &RBatis,
    column_name: &str,
) -> rbatis::Result<u64> {
    impled!()
}

///查询短网址是否存在
#[sql("select count(*) from fly_article_category where short_url = ?")]
pub async fn find_by_short_url_unique(
    rb: &RBatis,
    short_url: &str,
) -> rbatis::Result<u64> {
    impled!()
}






