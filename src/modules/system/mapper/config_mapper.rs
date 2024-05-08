use rbatis::{impl_select, impl_select_page, py_sql, RBatis};

use crate::modules::system::entity::config_entity::SystemConfig;
use crate::modules::system::entity::config_model::ConfigPageBO;

rbatis::crud!(SystemConfig {}, "fly_system_config");

/// 查询config_name的参数名称是否已存在
#[py_sql("
  `select count(*) from fly_system_config`
  ` where config_name = #{config_name} `
  if id != null:
    ` and id != #{id}`
  ")]
pub async fn find_by_name_unique(rb: &RBatis, config_name: Option<String>, id: Option<u64>, ) -> rbatis::Result<u64> {
    impled!()
}

/// 查询config_key是否已存在
#[py_sql("
  `select count(*) from fly_system_config`
  ` where config_key = #{config_key} `
  if id != null:
    ` and id != #{id}`
  ")]
pub async fn find_by_key_unique(rb: &RBatis, config_key: Option<String>, id: Option<u64>, ) -> rbatis::Result<u64> {
    impled!()
}

impl_select!(SystemConfig{select_by_key(config_key:&str) -> Option => "`where config_key = #{config_key} limit 1`"},"fly_system_config");

impl_select_page!(SystemConfig{select_config_page(item: ConfigPageBO) =>"
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
       order by create_time desc"},"fly_system_config");

