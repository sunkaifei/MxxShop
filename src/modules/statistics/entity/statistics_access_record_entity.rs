use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};


/// 访问记录
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct StatisticsAccessRecord {
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
