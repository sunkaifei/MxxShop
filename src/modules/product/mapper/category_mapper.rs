//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use rbatis::{crud, py_sql, RBatis};

use crate::modules::product::entity::category_entity::ProductCategory;



crud!(ProductCategory {}, "mxx_product_category");

//查询同父级类目下分类名称是否唯一
#[py_sql("
    `select count(*) from mxx_product_category where title_name = #{title_name}`
    ` and parent_id = #{parent_id}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_name_unique(rb: &RBatis, title_name: &Option<String>, parent_id: &Option<u64>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

///查询所有产品分类列表
#[py_sql("`select * from mxx_product_category order by sort desc`")]
pub async fn select_all_list(rb: &RBatis) -> rbatis::Result<Vec<ProductCategory>> {
    impled!()
}
