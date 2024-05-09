//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{crud, impl_select, impl_select_page};

use crate::modules::upload::entity::attach_entity::Attach;
use crate::modules::upload::entity::attach_model::AttachPageBO;

crud!(Attach {}, "fly_attach");

impl_select!(Attach{select_by_md5(md5:&str) -> Option => "`where md5 = #{md5} limit 1`"}, "fly_attach");


impl_select_page!(Attach{select_attach_page(item: AttachPageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.name != null && item.name != '':
          ` and name like '%${item.name}%'`
        choose:
          when item.status == 0:
            ` and status >= 0 `
          when item.status == 1:
            ` and status = 0 `
          when item.status == 2:
            ` and status = 1 `
    if !sql.contains('count'):
      order by id desc"},"fly_attach");

