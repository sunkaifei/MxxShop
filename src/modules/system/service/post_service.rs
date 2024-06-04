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
use crate::core::errors::error::{Error, Result};
use crate::modules::system::entity::post_entity::SystemPost;
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