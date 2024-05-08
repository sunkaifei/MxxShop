use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::modules::system::entity::config_entity::SystemConfig;
use crate::utils::string_utils::{string_to_u64,u64_to_string};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ConfigSaveRequest {
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
}

impl From<ConfigSaveRequest> for SystemConfig {
    fn from(item: ConfigSaveRequest) -> Self {
        Self {
            config_id: 0,
            config_name: item.config_name,
            config_key: item.config_key,
            config_value: item.config_value,
            config_type: item.config_type,
            remark: item.remark,
            sort: item.sort,
            create_by: None,
            create_time: Option::from(DateTime::now()),
            update_by: None,
            update_time: Option::from(DateTime::now()),
       }
   }
}


///更新系统配置
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ConfigUpdateRequest {
    ///参数主键
    #[serde(deserialize_with = "string_to_u64")]
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
}

impl From<ConfigUpdateRequest> for SystemConfig {
    fn from(item: ConfigUpdateRequest) -> Self {
        Self {
            config_id: item.config_id,
            config_name: item.config_name,
            config_key: item.config_key,
            config_value: item.config_value,
            config_type: item.config_type,
            remark: item.remark,
            sort: item.sort,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: Option::from(DateTime::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigPageRequest {
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    // 状态查询（0和空都是所有，1查询为0的数据，2查询为1的数据）
    pub config_type: Option<String>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigPageBO {
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub config_type: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}


impl From<ConfigPageRequest> for ConfigPageBO {
    fn from(req: ConfigPageRequest) -> Self {
        Self {
            config_name: req.config_name,
            config_key: req.config_key,
            config_type: req.config_type.map(|s| {
                s.parse::<i8>().unwrap_or_else(|_| {
                    0
                })
            }),
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConfigVO {
    ///参数主键
    #[serde(serialize_with = "u64_to_string")]
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
    ///更新时间
    pub update_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigVO {
    ///参数主键
    #[serde(serialize_with = "u64_to_string")]
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
    ///更新时间
    pub update_time: String,
}

impl From<SystemConfig> for ConfigVO {
    fn from(item: SystemConfig) -> Self {
        Self {
            config_id: item.config_id,
            config_name: item.config_name,
            config_key: item.config_key,
            config_value: item.config_value,
            config_type: item.config_type,
            remark: item.remark,
            sort: item.sort,
            update_time: item.update_time.map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default(),
        }
    }
}
