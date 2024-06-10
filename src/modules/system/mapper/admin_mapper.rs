//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{impl_select_page, py_sql, RBatis};

use crate::modules::system::entity::admin_entity::{SystemAdmin, AdminDeptsMerge, AdminRolesMerge, AdminPostMerge};
use crate::modules::system::entity::admin_model::{UserListDTO};

rbatis::crud!(SystemAdmin {}, "mxx_system_admin");

/// 查询用户名是否已存在
#[py_sql("
    `select count(*) from mxx_system_admin where user_name = #{user_name}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_name_unique(rb: &RBatis, user_name: &Option<String>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

/// 查询手机号是否已存在
#[py_sql("
    `select count(*) from mxx_system_admin where mobile = #{mobile}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_mobile_unique(rb: &RBatis, mobile: &Option<String>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

/// 查询邮箱是否已存在
#[py_sql("
    `select count(*) from mxx_system_admin where email = #{email}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_email_unique(rb: &RBatis, email: &Option<String>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

/// 查询昵称是否已存在
#[py_sql("
    `select count(*) from mxx_system_admin where nick_name = #{nick_name}`
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_by_nick_name_unique(rb: &RBatis, nick_name: &Option<String>, id: &Option<u64>) -> rbatis::Result<u64> {
    impled!()
}

impl_select_page!(SystemAdmin{select_by_page(item: &UserListDTO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if item.user_name != null && item.user_name != '':
          ` and user_name LIKE '%#{item.user_name}%' OR nick_name LIKE '%#{item.user_name}%' `
        if item.mobile != null && item.mobile != '':
          ` and mobile = #{item.mobile} `
        choose:
          when item.status == 0:
            ` and status >= 0 `
          when item.status == 1:
            ` and status = 0 `
          when item.status == 2:
            ` and status = 1 `
     if !sql.contains('count'):
       order by id desc"},"mxx_system_admin");

rbatis::pysql_select_page!(select_by_page(item: &UserListDTO) -> SystemAdmin =>
r#"
  if item.depts_id != '' && item.depts_id != null:
    `WITH RECURSIVE DeptTree AS (SELECT d1.id, d1.parent_id FROM mxx_system_depts d1 WHERE d1.id = #{item.depts_id} UNION ALL SELECT d2.id, d2.parent_id FROM mxx_system_depts d2 INNER JOIN DeptTree dt ON d2.parent_id = dt.id) `
  `select `
    if do_count == true:
     ` count(DISTINCT u.id) as count `
    if do_count == false:
     `DISTINCT u.*`
    ` from`
    if item.depts_id != '' && item.depts_id != null:
     ` mxx_system_depts d left join mxx_system_admin_depts_merge m on m.depts_id = d.id left join `
    ` mxx_system_admin u`
    if item.depts_id != '' && item.depts_id != null:
     ` on m.admin_id = u.id `
    ` where u.del_flag = 0`
    if item.admin_id != '' && item.admin_id != null:
     ` AND u.id = #{item.admin_id}`
    if item.user_name != '' && item.user_name != null:
     ` AND u.user_name like concat('%', #{item.user_name}, '%') or u.nick_name like concat('%', #{item.nick_name}, '%')`
    if item.status != '' && item.status != null:
     ` AND u.status = #{item.status}`
    if item.mobile != '' && item.mobile != null:
     ` AND u.mobile = #{item.mobile}`
    if item.begin_time != '' && item.begin_time != null:
     ` AND date_format(u.create_time,"%y%m%d") >= date_format(#{item.begin_time},"%y%m%d")`
    if item.end_time != '' && item.end_time != null:
     ` AND date_format(u.create_time,"%y%m%d") <= date_format(#{item.end_time},"%y%m%d")`
    if item.depts_id != '' && item.depts_id != null:
     ` AND m.depts_id IN (SELECT dt.id FROM DeptTree dt)`
    if !sql.contains('count'):
     ` order by u.id desc`
    if do_count == false:
     ` limit #{page_no}, #{page_size}`
    "#);

rbatis::crud!(AdminRolesMerge {}, "mxx_system_admin_role_merge");



rbatis::crud!(AdminDeptsMerge {}, "mxx_system_admin_depts_merge");


rbatis::crud!(AdminPostMerge {},"mxx_system_admin_post_merge");