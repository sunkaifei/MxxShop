//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use rbatis::{crud, impl_select_page, py_sql, RBatis};
use crate::modules::system::entity::post_entity::SystemPost;
use crate::modules::system::entity::post_model::{PostPageBO};

crud!(SystemPost {},"mxx_system_post");

/// 查询职位名称是否已存在
#[py_sql("
    `select count(*) from mxx_system_post  where name = #{name}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_name_unique(rb: &RBatis, name: Option<String>, id: Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

impl_select_page!(SystemPost{select_by_page(item: &PostPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.name != null && item.name != '':
          ` and name = #{item.name} `
        choose:
          when item.enabled == 0:
            ` and enabled >= 0 `
          when item.enabled == 1:
            ` and enabled = 0 `
          when item.enabled == 2:
            ` and enabled = 1 `
     if !sql.contains('count'):
       order by create_time desc"},"mxx_system_post");