//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!
//! 

use crate::core::errors::error::{Error, Result};
use crate::modules::system::entity::navigation_entity::Navigation;
use crate::modules::system::entity::navigation_model::{NavigationPageBO, NavigationSaveRequest};
use crate::modules::system::mapper::navigation_mapper;
use crate::pool;

pub async fn save_navigation(item: NavigationSaveRequest) -> Result<u64> {
    let navigation_entity: Navigation = item.into();

    let rows = Navigation::insert(pool!(), &navigation_entity).await?;
    Ok(rows.rows_affected)
}

pub async fn batch_delete_in_column(ids_vec: Vec<Option<String>>) -> Result<u64> {
    return if ids_vec.is_empty() {
        Err(Error::from("删除的ID不能为空".to_string()))
    } else {
        let result = Navigation::delete_in_column(pool!(), "id", &ids_vec).await?;
        Ok(result.rows_affected)
    }
}

pub async fn update_navigation(item: NavigationSaveRequest) -> Result<u64> {
    let navigation_entity: Navigation = item.into();

    let rows = Navigation::update_by_column(pool!(), &navigation_entity, "id").await?;
    Ok(rows.rows_affected)
}

pub async fn find_by_name(navigation_name: &Option<String>, parent_id: &Option<u64>, id: &Option<u64>) -> Result<bool> {
    let result = navigation_mapper::find_by_name_unique(pool!(), navigation_name, parent_id, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn get_by_detail(id: Option<u64>) -> rbatis::Result<Option<Navigation>> {
    Ok(Navigation::select_by_column(pool!(),"id", id).await?
        .into_iter()
        .next())
}

pub async fn select_all(item: &NavigationPageBO) -> rbatis::Result<Vec<Navigation>> {
    let result = navigation_mapper::select_all_list(pool!(), item).await;
    return result;
}