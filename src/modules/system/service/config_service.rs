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
use crate::core::web::entity::common::BathIdRequest;
use crate::modules::system::entity::config_entity::SystemConfig;
use crate::modules::system::entity::config_model::{ConfigPageBO, ConfigSaveRequest, ConfigUpdateRequest};
use crate::modules::system::mapper::{config_mapper};
use crate::pool;

pub async fn save_config(item: ConfigSaveRequest) -> Result<u64> {
    let mut config_entity :SystemConfig  = item.into();
    Ok(SystemConfig::insert(pool!(), &config_entity).await?.rows_affected)
}

pub async fn batch_delete(item: BathIdRequest) -> Result<u64> {
    return if let Some(ids_vec) = item.ids.clone() {
        if ids_vec.is_empty() {
            Err(Error::from("删除的ID不能为空!".to_string()))
        } else {
            let result = SystemConfig::delete_in_column(pool!(), "config_id", &ids_vec).await?;
            Ok(result.rows_affected)
        }
    } else {
        Err(Error::from("删除的ID不能为空!".to_string()))
    }
}

/// 更新配置信息
pub async fn update_config(item: ConfigUpdateRequest) -> Result<u64> {
    let mut config_entity :SystemConfig  = item.into();
    Ok(SystemConfig::update_by_column(pool!(), &config_entity, "config_id").await?.rows_affected)
}

/// 查询config_name的参数名称是否已存在
pub async fn find_by_name_unique(config_name: &Option<String>, id: &Option<u64>) -> Result<bool> {
    let result = config_mapper::find_by_name_unique(pool!(), config_name, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// 查询config_key是否已存在
pub async fn find_by_key_unique(config_key: &Option<String>, id: &Option<u64>) -> Result<bool> {
    let result = config_mapper::find_by_key_unique(pool!(), config_key, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}


///按id查询配置信息
pub async fn get_by_detail(id: &Option<u64>) -> rbatis::Result<Option<SystemConfig>> {
    let st = SystemConfig::select_by_column(pool!(), "config_id", id).await?
        .into_iter()
        .next();
    Ok(st)
}


///查询参数key信息
pub async fn select_by_key(config_key: &Option<String>) -> rbatis::Result<Option<SystemConfig>> {
    let result = SystemConfig::select_by_key(pool!(), config_key).await;
    return result;
}

pub async fn select_by_page(item: ConfigPageBO) -> rbatis::Result<Page<SystemConfig>> {
    let page_req = &PageRequest::new(item.page_num.clone().unwrap_or_default(), item.page_size.clone().unwrap_or_default());
    let result = SystemConfig::select_by_page(pool!(),page_req, item).await;
    Ok(result?)
}