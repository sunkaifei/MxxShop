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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpAddress {
    /// IP段ID
    pub id: Option<u64>,
    /// IP段起始
    pub start_ip: Option<u32>,
    /// IP段结束
    pub end_ip: Option<u32>,
    /// 国家
    pub country: Option<String>,
    /// 省份
    pub province: Option<String>,
    /// 市级地区
    pub city: Option<String>,
    /// 县
    pub county: Option<String>,
    /// 详细地址
    pub address: Option<String>,
    /// 地址描述
    pub local: Option<String>,
}