//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{Page, PageRequest};
use rbatis::rbdc::DateTime;
use crate::core::errors::error::{Error, Result};
use crate::modules::system::entity::admin_entity::{AdminPostMerge};
use crate::modules::system::entity::post_entity::{SystemPost};
use crate::modules::system::entity::post_model::{PostPageBO, PostSaveRequest, PostUpdateRequest};
use crate::modules::system::mapper::post_mapper;
use crate::pool;

/// 新增职位
pub async fn save_post(item: PostSaveRequest) -> Result<u64> {
    let post_entity: SystemPost = item.into();

    let rows = SystemPost::insert(pool!(), &post_entity).await?;
    Ok(rows.rows_affected)
}

pub async fn delete_in_column(ids_vec: Vec<Option<String>>) -> Result<u64> {
    return if ids_vec.is_empty() {
        Err(Error::from("删除的ID不能为空".to_string()))
    } else {
        let result = SystemPost::delete_in_column(pool!(), "id", &ids_vec).await?;
        Ok(result.rows_affected)
    }
}

/// 更新职位
pub async fn update_post(item: PostUpdateRequest) -> Result<u64> {
    let role_entity: SystemPost = item.into();

    let rows = SystemPost::update_by_column(pool!(), &role_entity, "id").await?;
    Ok(rows.rows_affected)
}

/// 批量更新职位和管理员关联关系
pub async fn batch_update_post(post_ids: &Option<Vec<Option<u64>>>, admin_id: &Option<u64>) -> Result<u64> {
    AdminPostMerge::delete_by_column(pool!(), "admin_id", admin_id).await?;
    let result = match post_ids {
        Some(post_vec) if !post_vec.is_empty() && !post_vec.iter().all(|item| item.is_none()) => {
            let sys_post_admin_list: Vec<AdminPostMerge> = post_vec
                .iter()
                .map(|post_id| AdminPostMerge {
                    id: None,
                    create_time: Some(DateTime::now()),
                    post_id: post_id.clone(),
                    admin_id: *admin_id,
                })
                .collect();
            AdminPostMerge::insert_batch(pool!(), &sys_post_admin_list, 20).await?.rows_affected
        }
        _ => 0,
    };
    Ok(result)
}

/// 根据名称查询职位是否唯一
pub async fn find_by_name(post_name: &Option<String>, id: &Option<u64>) -> Result<bool> {
    let result = post_mapper::find_by_name_unique(pool!(), post_name, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// 根据ID查询职位详情
pub async fn get_by_detail(id: &Option<u64>) -> rbatis::Result<Option<SystemPost>> {
    Ok(SystemPost::select_by_column(pool!(),"id", id).await?
        .into_iter()
        .next())
}

/// # 根据管理员ID查询关联的部门列表
/// ## admin_id: 用户id
///
/// * 返回值: 部门列表
///
pub async fn select_by_admin_id(admin_id: &Option<u64>) -> rbatis::Result<Vec<SystemPost>> {
    let result_merge = AdminPostMerge::select_by_column(pool!(), "admin_id", admin_id).await?;
    let id_list: Vec<Option<u64>> = result_merge.iter().map(|data| data.post_id).collect();
    if !id_list.is_empty() {
        Ok(SystemPost::select_in_column(pool!(), "id", &id_list).await?)
    } else {
        Ok(vec![])
    }
}

///查询所有职位列表
pub async fn select_all() -> rbatis::Result<Vec<SystemPost>> {
    let result = post_mapper::select_all_list(pool!()).await;
    return result;
}


pub async fn select_by_page(item: PostPageBO) -> rbatis::Result<Page<SystemPost>> {
    let page_req = &PageRequest::new(item.page_num.unwrap_or_default(), item.page_size.unwrap_or_default());
    let result = SystemPost::select_by_page(pool!(), page_req, &item).await;
    Ok(result?)
}