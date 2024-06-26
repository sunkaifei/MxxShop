//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{impl_select, impl_select_page, py_sql, RBatis};

use crate::modules::system::entity::config_entity::SystemConfig;
use crate::modules::system::entity::config_model::ConfigPageBO;

rbatis::crud!(SystemConfig {}, "mxx_system_config");

/// 查询config_name的参数名称是否已存在
#[py_sql("
  `select count(*) from mxx_system_config`
  ` where config_name = #{config_name} `
  if config_id != null:
    ` and config_id != #{config_id}`
  ")]
pub async fn find_by_name_unique(rb: &RBatis, config_name: &Option<String>, config_id: &Option<u64>, ) -> rbatis::Result<u64> {
    impled!()
}

/// 查询config_key是否已存在
#[py_sql("
  `select count(*) from mxx_system_config`
  ` where config_key = #{config_key} `
  if config_id != null:
    ` and config_id != #{config_id}`
  ")]
pub async fn find_by_key_unique(rb: &RBatis, config_key: &Option<String>, config_id: &Option<u64>, ) -> rbatis::Result<u64> {
    impled!()
}

impl_select!(SystemConfig{select_by_key(config_key :&Option<String>) -> Option => "`where config_key = #{config_key} limit 1`"},"mxx_system_config");

impl_select_page!(SystemConfig{select_by_page(item: ConfigPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.config_name != null && item.config_name != '':
          ` and config_name = #{item.config_name} `
        choose:
          when item.config_type == 0:
            ` and config_type >= 0 `
          when item.config_type == 1:
            ` and config_type = 0 `
          when item.config_type == 2:
            ` and config_type = 1 `
     if !sql.contains('count'):
       order by create_time desc"},"mxx_system_config");

