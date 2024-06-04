//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use serde::{Deserialize, Serialize};
use crate::utils::string_utils::{deserialize_string_to_u64};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BathIdRequest {
    pub ids: Option<Vec<Option<String>>>,
}

#[derive(Deserialize)]
pub struct InfoId {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
}

#[derive(Deserialize)]
pub struct QueryUrl {
    pub short_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteIntIdRequest {
    pub ids: Vec<i32>,
}

#[derive(serde::Serialize)]
pub struct Pagination {
    ///当前页码
    pub current: u64,
    ///总页数
    pub total: u64,
    ///每页数量
    pub page_size: u64,
}