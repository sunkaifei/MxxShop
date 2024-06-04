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
use crate::modules::system::entity::system_log_entity::SystemLog;
use crate::modules::system::entity::system_log_model::LogPageBO;

crud!(SystemLog {},"mxx_system_log");

impl_select_page!(SystemLog{select_by_page(item: LogPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.title != null && item.title != '':
          ` and title = #{item.title} `
        if item.business_type != null && item.business_type != '':
          ` and business_type = #{item.business_type} `
        if item.operator_type != null && item.operator_type != '':
          ` and operator_type = #{item.operator_type} `
        if item.begin_time != null && item.begin_time != '':
          ` and date_format(create_time,'%y%m%d') >= date_format(#{item.begin_time},'%y%m%d')`
        if item.end_time != null && item.end_time != '':
          ` and date_format(create_time,'%y%m%d') <= date_format(#{item.end_time},'%y%m%d')`
        choose:
          when item.status == 0:
            ` and status >= 0 `
          when item.status == 1:
            ` and status = 0 `rbatis
          when item.status == 2:
            ` and status = 1 `
     if !sql.contains('count'):
       order by id desc"},"mxx_system_log");