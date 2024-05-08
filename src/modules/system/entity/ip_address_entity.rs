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