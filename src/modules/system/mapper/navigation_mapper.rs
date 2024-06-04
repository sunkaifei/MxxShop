//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!
use rbatis::{py_sql, RBatis};
use crate::modules::system::entity::navigation_entity::Navigation;
use crate::modules::system::entity::navigation_model::NavigationPageBO;

///增删改查导航菜单
rbatis::crud!(Navigation {}, "mxx_navigation");

/// 查询导航名称是否已存在
#[py_sql("
    `select count(*) from mxx_navigation  where title_name = #{title_name}`
    ` and parent_id = #{parent_id}`
    if id != null:
      ` and id != #{id}`
    ")]
pub async fn find_by_name_unique(rb: &RBatis, title_name: &Option<String>, parent_id: &Option<u64>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

#[py_sql("
    `select * from mxx_navigation `
     trim end=' where ':
       ` where `
       trim ' and ':
         if item.title_name != null && item.title_name != '':
           ` and title_name = #{item.title_name} `
         choose:
           when item.is_show == 0:
             ` and is_show >= 0 `
           when item.is_show == 1:
             ` and is_show = 0 `
           when item.is_show == 2:
             ` and is_show = 1 `
           otherwise:
            ' and is_show >= 0 '
     `order by sort desc`
    ")]
pub async fn select_all_list(rb: &RBatis, item: &NavigationPageBO) -> rbatis::Result<Vec<Navigation>> {
    impled!()
}

