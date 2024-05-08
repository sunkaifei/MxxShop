use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    ///配置id
    pub config_id: u64,
    ///参数名称
    pub config_name: Option<String>,
    ///参数键名
    pub config_key: Option<String>,
    ///参数键值
    pub config_value: Option<String>,
    ///系统内置（Y是 N否）
    pub config_type: Option<String>,
    ///备注
    pub remark: Option<String>,
    ///排序
    pub sort: Option<i32>,
    ///创建者
    pub create_by: Option<String>,
    ///创建时间
    pub create_time: Option<DateTime>,
    ///更新者
    pub update_by: Option<String>,
    ///更新时间
    pub update_time: Option<DateTime>,
}
