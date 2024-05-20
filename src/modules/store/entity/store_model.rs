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
use crate::modules::store::entity::store_entity::Store;
use crate::utils::string_utils::{deserialize_string_to_u64,serialize_option_u64_to_string};


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct StoreSaveRequest {
    // 会员Id
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: Option<u64>,
    // 会员名称
    pub user_name: Option<String>,
    // 是否自营
    pub self_operated: Option<bool>,
    // 店铺名称
    pub store_name: Option<String>,
    // 店铺状态
    pub store_disable: Option<i8>,
    // 店铺logo
    pub store_logo: Option<String>,
    // 详细地址
    pub store_address_detail: Option<String>,
    // 地址id
    pub store_address_id_path: Option<String>,
    // 地址名称
    pub store_address_path: Option<String>,
    // 经纬度
    pub store_center: Option<String>,
    // 店铺简介
    pub store_desc: Option<String>,
    // 默认页面是否开启
    pub page_show: Option<bool>,
    // 是否开启自提
    pub self_pick_flag: Option<bool>,
    // 创建者
    pub create_by: Option<String>,
}

impl From<StoreSaveRequest> for Store {
    fn from(item: StoreSaveRequest) -> Self {
        Self {
            id: None,
            user_id: item.user_id,
            user_name: item.user_name,
            self_operated: item.self_operated,
            store_name: item.store_name,
            store_disable: item.store_disable,
            store_end_time: None,
            store_logo: item.store_logo,
            store_address_detail: item.store_address_detail,
            store_address_id_path: item.store_address_id_path,
            store_address_path: item.store_address_path,
            store_center: None,
            store_desc: None,
            page_show: None,
            self_pick_flag: None,
            create_by: None,
            create_time: Option::from(DateTime::now()),
            update_by: None,
            update_time: None,
            delete_flag: None,
        }
   }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct StoreUpdateRequest {
    ///店铺id
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    // 会员Id
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: Option<u64>,
    // 会员名称
    pub user_name: Option<String>,
    // 是否自营
    pub self_operated: Option<bool>,
    // 店铺名称
    pub store_name: Option<String>,
    // 店铺状态
    pub store_disable: Option<i8>,
    // 店铺关闭时间
    pub store_end_time: Option<DateTime>,
    // 店铺logo
    pub store_logo: Option<String>,
    // 详细地址
    pub store_address_detail: Option<String>,
    // 地址id
    pub store_address_id_path: Option<String>,
    // 地址名称
    pub store_address_path: Option<String>,
    // 经纬度
    pub store_center: Option<String>,
    // 店铺简介
    pub store_desc: Option<String>,
    // 默认页面是否开启
    pub page_show: Option<bool>,
    // 是否开启自提
    pub self_pick_flag: Option<bool>,
    // 更新者
    pub update_by: Option<String>,
    // 更新时间
    pub update_time: Option<DateTime>,
    // 删除标志 true/false 删除/未删除
    pub delete_flag: Option<bool>,
}

impl From<StoreUpdateRequest> for Store {
    fn from(item: StoreUpdateRequest) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            user_name: item.user_name,
            self_operated: item.self_operated,
            store_name: item.store_name,
            store_disable: item.store_disable,
            store_end_time: item.store_end_time,
            store_logo: item.store_logo,
            store_address_detail: None,
            store_address_id_path: None,
            store_address_path: None,
            store_center: None,
            store_desc: None,
            page_show: None,
            self_pick_flag: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: Option::from(DateTime::now()),
            delete_flag: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryStorePageDTO {
    /// 是否自营
    pub self_operated: Option<bool>,
    /// 店铺名称
    pub store_name: Option<String>,
    /// 店铺状态
    pub store_disable: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 删除标志 true/false 删除/未删除
    pub delete_flag: Option<bool>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}