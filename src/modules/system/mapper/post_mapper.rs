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
    `select count(*) from mxx_system_post  where post_name = #{post_name}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_name_unique(rb: &RBatis, post_name: &Option<String>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

///查询所有职位列表
#[py_sql("`select * from mxx_system_post order by sort desc`")]
pub async fn select_all_list(rb: &RBatis) -> rbatis::Result<Vec<SystemPost>> {
    impled!()
}

impl_select_page!(SystemPost{select_by_page(item: &PostPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.post_name != null && item.post_name != '':
          ` and post_name = #{item.post_name} `
        choose:
          when item.enabled == 0:
            ` and enabled >= 0 `
          when item.enabled == 1:
            ` and enabled = 0 `
          when item.enabled == 2:
            ` and enabled = 1 `
     if !sql.contains('count'):
       order by sort desc"},"mxx_system_post");