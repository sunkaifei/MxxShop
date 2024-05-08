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






