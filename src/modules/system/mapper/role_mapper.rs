//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::collections::HashMap;

use rbatis::{crud, impl_select_page, py_sql, sql};
use rbatis::RBatis;

use crate::modules::system::entity::role_entity::SystemRole;
use crate::modules::system::entity::role_menu_entity::SystemRoleMenu;

crud!(SystemRole {},"mxx_system_role");

/// 查询角色名称是否已存在
#[py_sql("
    `select count(*) from mxx_system_role`
    ` where role_name = #{role_name} `
     if id != null:
        ` and id != #{id}`
    ")]
pub async fn find_role_by_name_unique(rb: &RBatis, role_name: &Option<String>, id: &Option<u64>, ) -> rbatis::Result<u64> {
    impled!()
}

// 分页查询角色信息
impl_select_page!(SystemRole{select_page(role_name:&str,status:&str) =>"
     trim end=' where ':
       ` where `
       trim ' and ':
         if role_name != null && role_name != '':
           ` and role_name = #{role_name} `
         choose:
           when status == 0:
             ` and status >= 0 `
           when status == 1:
             ` and status = 0 `
           when status == 2:
             ` and status = 1 `
     if !sql.contains('count'):
       ` order by sort asc`"},"mxx_system_role");

//增删改查菜单与角色关联
rbatis::crud!(SystemRoleMenu {}, "mxx_system_role_menus");

/// 查询角色和菜单是否已关联
#[py_sql("select count(*) from mxx_system_role_menus where role_id = #{role_id} and menu_id = #{menu_id}")]
pub async fn find_merge_by_unique(rb: &RBatis, role_id: Option<u64>, menu_id: Option<u64>, ) -> rbatis::Result<u64> {
    impled!()
}

///角色id查询所有关联的菜单id
#[sql("select * from mxx_system_role_menus where role_id = ?")]
pub async fn get_merge_by_role_id(rb: &RBatis,role_id: &Option<u64>,) -> rbatis::Result<Vec<SystemRoleMenu>> {
    impled!()
}

///查询当前菜单id关联的所有角色ID
#[sql("select role_id from mxx_system_role_menus where menu_id = ?")]
pub async fn query_menu_by_role(rb: &RBatis, menu_id: Option<u64>, ) -> rbatis::Result<Vec<HashMap<String, u64>>> {
    impled!()
}

#[sql("select r.id, r.role_name, r.role_key, r.sort, r.data_scope, r.status, r.del_flag
        from mxx_system_role r
		    left join mxx_system_admin_role ur on r.id = ur.role_id
		where ur.admin_id = ?")]
pub async fn query_admin_by_role(rb: &RBatis, admin_id: u64, ) -> rbatis::Result<Vec<SystemRole>> {
    impled!()
}

///查询当前用户权限
#[sql("select distinct m.perms
		from mxx_system_menu m
			 left join mxx_system_role_menus rm on m.id = rm.menu_id
			 left join mxx_system_admin_role ur on rm.role_id = ur.role_id
			 left join mxx_system_role ro on ur.role_id = ro.id
			 left join mxx_system_admin u on ur.admin_id = u.id
		where u.id = ? and ro.status = 0 and ro.status = 0")]
pub async fn get_admin_by_role(rb: &RBatis, admin_id: u64) -> rbatis::Result<Vec<String>> {
    impled!()
}