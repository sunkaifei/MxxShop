//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use crate::core::web::entity::common::BathIdRequest;

use crate::core::errors::error::{Error, Result};
use crate::modules::system::entity::admin_entity::AdminDeptsMerge;
use crate::modules::system::entity::depts_entity::SystemDept;
use crate::modules::system::entity::depts_model::{DeptPageDTO, DeptPageRequest, DeptSaveRequest, DeptTree, DeptTreeData, DeptUpdateRequest};
use crate::modules::system::mapper::depts_mapper;
use crate::pool;
use crate::utils::snowflake_id::generate_snowflake_id;

pub async fn add_dept(payload: DeptSaveRequest) -> Result<u64> {
    let mut menu_entity: SystemDept = payload.into();
    menu_entity.id = Option::from(generate_snowflake_id());
    return Ok(SystemDept::insert(pool!(), &menu_entity).await.unwrap_or_default().rows_affected)
}

pub async fn delete_in_column(item: BathIdRequest) -> Result<u64> {
    if let Some(ids_vec) = item.ids.clone() {
        if ids_vec.is_empty() {
            return Err(Error::from("删除的ID不能为空!".to_string()));
        } else {
            //有下级的时候 不能直接删除
            let menus = SystemDept::select_in_column(pool!(), "parent_id", &ids_vec)
                .await
                .unwrap_or_default();

            if menus.len() > 0 {
                return Err(Error::from("有下级菜单,不能直接删除!".to_string()));
            }

            let result = SystemDept::delete_in_column(pool!(), "id", &ids_vec).await?;
            return Ok(result.rows_affected);
        }
    }else {
        return Err(Error::from("删除的ID不能为空!".to_string()));
    }

}

/// 修改部门信息
pub async fn update_dept(payload: DeptUpdateRequest) -> Result<u64> {
    let menu_entity: SystemDept = payload.into();
    let result = SystemDept::update_by_column(pool!(), &menu_entity,"id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}




// 用户部门组转树
pub fn dept_arr_to_tree(re_list: &mut Vec<DeptTree>, ori_arr: Vec<SystemDept>, pid: Option<u64>) {
    for (_, it) in ori_arr.iter().enumerate() {
        if pid == it.parent_id {
            let mut children = Vec::<DeptTree>::new();
            dept_arr_to_tree(&mut children, ori_arr.clone(), it.id);

            let temp_router = DeptTree {
                id: it.id.clone(),
                label: it.dept_name.clone().unwrap_or_default(),
                children: (|| -> Option<Vec<DeptTree>> {
                    if children.len() > 0 {
                        Some(children)
                    } else {
                        None
                    }
                })(),

                is_disabled: if it.status == Option::from(1) { true } else { false },
            };
            re_list.push(temp_router)
        }
    }
}

pub async fn all_dept_list_tree() -> rbatis::Result<Vec<DeptTree>> {
    let list: Vec<SystemDept> = SystemDept::select_all(pool!()).await?;
    let mut dept_list = Vec::<DeptTree>::new();
    dept_arr_to_tree(&mut dept_list, list, Option::from(0));
    Ok(dept_list)
}

// 路由数组转树
pub fn dept_list_to_tree(re_list: &mut Vec<DeptTreeData>, ori_arr: Vec<SystemDept>, pid: Option<u64>) {
    let default_pid = if pid.is_none() {
        ori_arr.iter().map(|dept| dept.parent_id).min().unwrap_or_default()
    } else {
        pid
    };

    for (_, it) in ori_arr.iter().enumerate() {
        if default_pid == it.parent_id {
            let mut children = Vec::<DeptTreeData>::new();
            dept_list_to_tree(&mut children, ori_arr.clone(), it.id);
            let temp_router = DeptTreeData {
                id: it.id.clone(),
                parent_id: it.parent_id.clone(),
                dept_name: it.dept_name.clone(),
                sort: it.sort.clone(),
                leader: it.leader.clone(),
                phone: it.phone.clone(),
                email: it.email.clone(),
                status: it.status,
                update_time: it.update_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
                children: if children.is_empty() { None } else { Some(children) },
            };
            re_list.push(temp_router)
        }
    }
}

// 查询部门详情
pub async fn get_by_detail(id: &Option<u64>) -> rbatis::Result<Option<SystemDept>> {
    let st = SystemDept::select_by_column(pool!(),"id", id).await?
           .into_iter()
           .next();
    Ok(st)
}

/// 根据管理员ID查询关联的查询部门列表
pub async fn select_by_ids(ids: &Vec<Option<u64>>) -> rbatis::Result<Vec<SystemDept>> {
    let result_merge = AdminDeptsMerge::select_in_column(pool!(), "admin_id", ids).await?;
    let id_list: Vec<Option<u64>> = result_merge.iter().map(|data| data.depts_id).collect();
    Ok(SystemDept::select_in_column(pool!(), "id", &id_list).await?)
}

// 查询部门所有数据列表
pub async fn select_all_list(item: DeptPageRequest) -> rbatis::Result<Vec<DeptTreeData>> {
    let dept_page:DeptPageDTO = item.into();
    let list: Vec<SystemDept> = depts_mapper::select_all_list(pool!(), &dept_page).await?;
    let mut dept_list = Vec::<DeptTreeData>::new();
    if dept_page.dept_name.is_some() || dept_page.status.is_some() {
        dept_list = list.into_iter().map(|dept| {
            DeptTreeData {
                id: dept.id.clone(),
                parent_id: dept.parent_id.clone(),
                dept_name: dept.dept_name.clone(),
                sort: dept.sort.clone(),
                leader: dept.leader.clone(),
                phone: dept.phone.clone(),
                email: dept.email.clone(),
                status: dept.status,
                update_time: dept.update_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
                children: None,
            }
        }).collect();
    } else {
        dept_list_to_tree(&mut dept_list, list, Option::from(0));
    };
    
    Ok(dept_list)
}