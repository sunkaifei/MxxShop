//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Store {
    /// ID
    pub id: Option<u64>,
    /// 会员Id
    pub user_id: Option<u64>,
    /// 会员名称
    pub user_name: Option<String>,
    /// 是否自营
    pub self_operated: Option<bool>,
    /// 店铺名称
    pub store_name: Option<String>,
    /// 店铺状态
    pub store_disable: Option<i8>,
    /// 店铺关闭时间
    pub store_end_time: Option<DateTime>,
    /// 店铺logo
    pub store_logo: Option<String>,
    /// 详细地址
    pub store_address_detail: Option<String>,
    /// 地址id
    pub store_address_id_path: Option<String>,
    /// 地址名称
    pub store_address_path: Option<String>,
    /// 经纬度
    pub store_center: Option<String>,
    /// 店铺简介
    pub store_desc: Option<String>,
    /// 默认页面是否开启
    pub page_show: Option<bool>,
    /// 是否开启自提
    pub self_pick_flag: Option<bool>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<DateTime>,
    /// 删除标志 true/false 删除/未删除
    pub delete_flag: Option<bool>,
}



