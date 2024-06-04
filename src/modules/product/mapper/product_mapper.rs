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
use crate::modules::product::entity::product_entity::Product;
use crate::modules::product::entity::product_model::{ProductPageBO};

crud!(Product {}, "mxx_product");

//查询菜单名称是否唯一
#[py_sql("
    `select count(*) from mxx_product where product_name = #{product_name}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_name_unique(rb: &RBatis, product_name: &Option<String>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

impl_select_page!(Product{select_by_page(dto: &ProductPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.product_name != null && dto.product_name != '':
          ` and product_name like '%${dto.product_name}%'`
        if dto.cate_id != null && dto.cate_id != '':
          ` and cate_id = #{dto.cate_id} `
        choose:
          when dto.status == 0:
            ` and status >= 0 `
          when dto.status == 1:
            ` and status = 0 `
          when dto.status == 2:
            ` and status = 1 `
    if !sql.contains('count'):
      ` order by id desc `"},"mxx_product");