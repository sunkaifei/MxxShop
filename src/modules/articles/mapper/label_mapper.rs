//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{crud, impl_select_page, sql};
use rbatis::RBatis;

use crate::modules::articles::entity::label_entity::{Label, LabelMerge};
use crate::modules::articles::entity::label_model::LabelPageBO;

crud!(Label {}, "fly_article_label");

crud!(LabelMerge {}, "fly_article_label_merge");

#[sql("select count(*) from fly_article_label where short_url = ?")]
pub async fn find_by_short_url_unique(
    rb: &RBatis,
    short_url: &str,
) -> rbatis::Result<u64> {
    impled!()
}

#[sql("select count(*) from fly_article_label where title = ?")]
pub async fn find_by_title_unique(
    rb: &RBatis,
    title: &Option<String>,
) -> rbatis::Result<u64> {
    impled!()
}

impl_select_page!(Label{select_comment_page(item: &LabelPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.title != null && item.title != '':
          ` and title = #{item.title} `
        choose:
          when item.status == 0:
            ` and status >= 0 `
          when item.status == 1:
            ` and status = 0 `
          when item.status == 2:
            ` and status = 1 `
    if !sql.contains('count'):
      order by create_time desc"},"fly_article_label");