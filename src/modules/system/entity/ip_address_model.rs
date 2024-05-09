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
use crate::modules::system::entity::ip_address_entity::IpAddress;
use crate::utils::string_utils::{serialize_option_u64_to_string,deserialize_string_to_u64};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IpAddressSaveRequest {
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

impl From<IpAddressSaveRequest> for IpAddress {
    fn from(req: IpAddressSaveRequest) -> Self {
        Self {
            id: Option::from(0),
            start_ip: req.start_ip,
            end_ip: req.end_ip,
            country: req.country,
            province: req.province,
            city: req.city,
            county: req.county,
            address: req.address,
            local: req.local,
        }
   }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IpAddressUpdateRequest {
    /// IP段ID
    #[serde(deserialize_with = "deserialize_string_to_u64")]
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

impl From<IpAddressUpdateRequest>  for IpAddress{
    fn from(req: IpAddressUpdateRequest) -> Self {
        Self {
            id: req.id,
            start_ip: req.start_ip,
            end_ip: req.end_ip,
            country: req.country,
            province: req.province,
            city: req.city,
            county: req.county,
            address: req.address,
            local: req.local,
        }
    }
    
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpAddressPageDTO {
    pub start_ip: Option<String>,
    pub end_ip: Option<String>,
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
    // 当前页码数
    pub page_num: Option<u64>,
    // 每页条数
    pub page_size: Option<u64>,
}

impl From<QueryPageRequest> for IpAddressPageDTO {
    fn from(request: QueryPageRequest) -> Self {
        Self {
            start_ip: request.start_ip,
            end_ip: request.end_ip,
            country: request.country,
            province: request.province,
            city: request.city,
            county: request.county,
            address: request.address,
            local: request.local,
            page_num: request.page_num,
            page_size: request.page_size,
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryPageRequest {
    /// IP段起始
    pub start_ip: Option<String>,
    /// IP段结束
    pub end_ip: Option<String>,
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
    // 当前页码数
    pub page_num: Option<u64>,
    // 每页条数
    pub page_size: Option<u64>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpListVO {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id:  Option<u64>,
    /// IP段起始
    pub start_ip: Option<String>,
    /// IP段结束
    pub end_ip: Option<String>,
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