use serde::{Deserialize, Serialize};

/// 统计来源
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct StatisticsSource {
    /// id
    pub id: Option<u64>,
    /// 统计日期（格式:yyyy-MM-dd）
    pub statistics_day: Option<String>,
    /// 来源网站类型 （1-搜索引擎  2-外部链接  3-直接访问）
    pub sorce_url_type: Option<i16>,
    /// 是否新客户 （0-否   1-是）
    pub is_new_visitor: Option<i8>,
    /// 访客设备类型（1-计算机   2-移动设备）
    pub visitor_device_type: Option<i16>,
    /// 来源域名
    pub source_domain: Option<Option<String>>,
    /// 来源外部链接网站地址或网站名称（如：百度/http://www.jeecms.com）
    pub sorce_url: Option<Option<String>>,
    /// 搜索引擎
    pub engine_name: Option<Option<String>>,
    /// 浏览量
    pub pvs: Option<i32>,
    /// 访客数
    pub uvs: Option<i32>,
    /// ip数
    pub ips: Option<i32>,
    /// 总访问时长(单位：秒)
    pub access_houre_long: Option<i32>,
    /// 只访问一次页面的访问次数
    pub only_one_pv: Option<i32>,
    /// 时间段
    pub statistics_hour: Option<i8>,
    /// 删除标识
    pub deleted: Option<i8>,
}