//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{crud, impl_select_page};

use crate::modules::articles::entity::comment_entity::Comment;
use crate::modules::articles::entity::comment_model::CommentPageBO;

crud!(Comment {}, "mxx_article_comment");

impl_select_page!(Comment{select_comment_page(item: &CommentPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.user_id != null && item.user_id != '':
          ` and user_id = #{item.user_id} `
        choose:
          when item.status == 0:
            ` and status >= 0 `
          when item.status == 1:
            ` and status = 0 `
          when item.status == 2:
            ` and status = 1 `
    if !sql.contains('count'):
      order by create_time desc"},"mxx_article_comment");










