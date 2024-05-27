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

use crate::modules::search::entity::store_entity::Store;
use crate::modules::search::entity::store_model::{QueryStorePageDTO};


crud!(Store {}, "mxx_store");

// 分页查询
impl_select_page!(Store{select_page(dto: &QueryStorePageDTO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.store_name != null && dto.store_name != '':
          ` and store_name = #{dto.store_name} `
        if dto.code != null && dto.code != '':
          ` and code = #{dto.code} `
        if dto.is_admin != null && dto.is_admin != '':
          ` and is_admin = #{dto.is_admin} `
        choose:
          when dto.store_disable == 0:
            ` and store_disable >= 0 `
          when dto.store_disable == 1:
            ` and store_disable = 0 `
          when dto.store_disable == 2:
            ` and store_disable = 1 `
    if !sql.contains('count'):
      ` order by id desc `"},"mxx_store");