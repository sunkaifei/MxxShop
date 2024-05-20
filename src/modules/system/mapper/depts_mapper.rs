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

use crate::modules::system::entity::dept_entity::SystemDept;
use crate::modules::system::entity::dept_model::DeptPageBO;

//增删改查菜单
rbatis::crud!(SystemDept {}, "fly_system_depts");

/// 查询同目录级别下部门是否已存在
#[py_sql("
    `select count(*) from fly_system_depts`
    ` where dept_name = #{dept_name} `
    `and parent_id = #{parent_id} `
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_dept_name_unique(rb: &RBatis, dept_name: Option<String>, parent_id: Option<u64>,  id: Option<u64>, ) -> rbatis::Result<u64> {
    impled!()
}

#[py_sql("
    `select * from fly_system_depts `
     trim end=' where ':
       ` where `
       trim ' and ':
         if item.dept_name != null && item.dept_name != '':
           ` and dept_name = #{item.dept_name} `
         choose:
           when item.status == 0:
             ` and status >= 0 `
           when item.status == 1:
             ` and status = 0 `
           when item.status == 2:
             ` and status = 1 `
           otherwise:
            ' and status >= 0 '
     `order by sort desc`
    ")]
pub async fn select_dept_list(rb: &RBatis, item: &DeptPageBO) -> rbatis::Result<Vec<SystemDept>> {
    impled!()
}


