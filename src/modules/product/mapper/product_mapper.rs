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
use crate::modules::product::entity::product_entity::Product;
use crate::modules::product::entity::product_model::{ProductPageDTO};

crud!(Product {}, "mxx_product");


impl_select_page!(Product{select_by_page(dto: &ProductPageDTO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.store_name != null && dto.store_name != '':
          ` and store_name like '%${dto.store_name}%'`
        if dto.dict_type != null && dto.dict_type != '':
          ` and dict_type = #{dto.dict_type} `
        choose:
          when dto.status == 0:
            ` and status >= 0 `
          when dto.status == 1:
            ` and status = 0 `
          when dto.status == 2:
            ` and status = 1 `
    if !sql.contains('count'):
      ` order by id desc `"},"mxx_product");