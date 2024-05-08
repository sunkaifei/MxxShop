use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::modules::statistics::entity::statistics_access_record_entity::StatisticsAccessRecord;

/// 访问记录
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordSaveRequest{
    pub source_url: Option<String>,
    pub access_url: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct StatisticsAccessRecordDTO {
    // id
    pub id: Option<u64>,
    // 是否登录访问（0-否  1-是）
    pub is_login: Option<i8>,
    // 登录用户id
    pub login_user_id: Option<u64>,
    // 登录用户名
    pub login_user_name: Option<String>,
    // 会话标识
    pub session_id: Option<String>,
    // cookie标识
    pub cookie_id: Option<String>,
    // 访问来源(1:PC  2:移动端H5  3:微信客户端H5 4:IOS 5:安卓 6:小程序)
    pub access_source_client: Option<i16>,
    // 访问网址
    pub access_url: Option<String>,
    // 来源网址
    pub source_url: Option<String>,
    // 来源域名
    pub source_domain: Option<String>,
    // 来源网站类型 （1-搜索引擎  2-外部链接  3-直接访问）
    pub source_url_type: Option<i16>,
    // 访客ip
    pub access_ip: Option<String>,
    // 访客设备系统（如：Win10 Mac10  Android8）
    pub access_device: Option<String>,
    // 访客浏览器类型
    pub access_browser: Option<String>,
    // 访客所属省份
    pub access_province: Option<String>,
    // 访客所属城市
    pub access_city: Option<String>,

    // 访客所属国家
    pub access_country: Option<String>,
    // 搜索名称
    pub engine_name: Option<String>,
    // 是否新访客（0:否   1:是）
    pub is_new_visitor: Option<i8>,
    // 设备类型
    pub device_type: Option<i32>,
    // 创建时间
    pub create_time: Option<DateTime>,
    // 删除标识
    pub deleted: Option<i8>,
}


impl From<StatisticsAccessRecordDTO> for StatisticsAccessRecord {
    fn from(s: StatisticsAccessRecordDTO) -> Self {
        Self {
            id: s.id,
            is_login: s.is_login,
            login_user_id: s.login_user_id,
            login_user_name: s.login_user_name,
            session_id: s.session_id,
            cookie_id: s.cookie_id,
            access_source_client: s.access_source_client,
            access_url: s.access_url,
            source_url: s.source_url,
            source_domain: s.source_domain,
            source_url_type: s.source_url_type,
            access_ip: s.access_ip,
            access_device: s.access_device,
            access_browser: s.access_browser,
            access_province: s.access_province,
            access_city: s.access_city,
            access_country: s.access_country,
            engine_name: s.engine_name,
            is_new_visitor: s.is_new_visitor,
            device_type: s.device_type,
            create_time: Option::from(DateTime::now()),
            deleted: s.deleted,
        }
    }
}