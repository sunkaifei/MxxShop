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
use crate::core::errors::error::Result;
use crate::modules::system::entity::config_entity::SystemConfig;
use crate::modules::system::entity::config_model::{ConfigPageBO, ConfigSaveRequest};
use crate::pool;
use crate::utils::snowflake_id::generate_snowflake_id;

pub async fn save_config(item: ConfigSaveRequest) -> Result<u64> {
    let mut config_entity :SystemConfig  = item.into();
    config_entity.config_id = generate_snowflake_id();
    Ok(SystemConfig::insert(pool!(), &config_entity).await?.rows_affected)
}

///按id查询配置信息
pub async fn get_config_by_id(id: u64) -> rbatis::Result<Option<SystemConfig>> {
    let st = SystemConfig::select_by_column(pool!(), "id", id).await?
        .into_iter()
        .next();
    Ok(st)
}


///查询参数key信息
pub async fn select_by_key(config_key: &str) -> rbatis::Result<Option<SystemConfig>> {
    let result = SystemConfig::select_by_key(pool!(), &config_key).await;
    return result;
}

pub async fn select_config_page(item: ConfigPageBO) -> rbatis::Result<Page<SystemConfig>> {
    let page_req = &PageRequest::new(item.page_num.clone(), item.page_size.clone());
    let result = SystemConfig::select_config_page(pool!(),page_req, item).await;
    Ok(result?)
}