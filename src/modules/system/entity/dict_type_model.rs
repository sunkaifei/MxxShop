use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::modules::system::entity::dict_type_entity::DictType;
use crate::utils::string_utils::{string_to_u64,u64_to_string};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeSaveRequest {
    // 字典主键
    #[serde(deserialize_with = "string_to_u64")]
    pub id: u64,
    // 字典名称
    pub dict_name: Option<String>,
    // 字典类型
    pub dict_type: Option<String>,
    // 排序
    pub sort: Option<i32>,
    // 状态（0正常 1停用）
    pub status: Option<i8>,
    // 创建者
    pub create_by: Option<String>,
    // 更新者
    pub update_by: Option<String>,
    // 备注
    pub remark: Option<String>,
}

impl From<DictTypeSaveRequest> for DictType {
    fn from(req: DictTypeSaveRequest) -> Self {
        Self {
            id: req.id,
            dict_name: req.dict_name,
            dict_type: req.dict_type,
            sort: req.sort,
            status: req.status,
            create_by: req.create_by,
            create_time: Option::from(DateTime::now()),
            update_by: req.update_by,
            update_time: Option::from(DateTime::now()),
            remark: req.remark,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeUpdateRequest {
    // 字典主键
    #[serde(deserialize_with = "string_to_u64")]
    pub id: u64,
    // 字典名称
    pub dict_name: Option<String>,
    // 字典类型
    pub dict_type: Option<String>,
    // 排序
    pub sort: Option<i32>,
    // 状态（0正常 1停用）
    pub status: Option<i8>,
    // 创建者
    pub create_by: Option<String>,
    // 更新者
    pub update_by: Option<String>,
    // 备注
    pub remark: Option<String>,
}

impl From<DictTypeUpdateRequest> for DictType {
    fn from(req: DictTypeUpdateRequest) -> Self {
        Self {
            id: req.id,
            dict_name: req.dict_name,
            dict_type: req.dict_type,
            sort: req.sort,
            status: req.status,
            create_by: req.create_by,
            create_time: Option::from(DateTime::now()),
            update_by: req.update_by,
            update_time: Option::from(DateTime::now()),
            remark: req.remark,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DictTypePageRequest {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    // 状态查询（0和空都是所有，1查询为0的数据，2查询为1的数据）
    pub status: Option<String>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DictTypePageBO {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub status: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}


impl From<DictTypePageRequest> for DictTypePageBO {
    fn from(req: DictTypePageRequest) -> Self {
        Self {
            dict_name: req.dict_name,
            dict_type: req.dict_type,
            status: req.status.map(|s| {
                    s.parse::<i8>().unwrap_or_else(|_| {
                        0
                    })
                }),
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}

///字典类型分页响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DictTypePageVO {
    #[serde(serialize_with = "u64_to_string")]
    pub id: u64,
    // 字典名称
    pub dict_name: Option<String>,
    // 字典类型
    pub dict_type: Option<String>,
    // 排序
    pub sort: Option<i32>,
    // 状态（0正常 1停用）
    pub status: Option<i8>,
    // 备注
    pub remark: Option<String>,
    ///更新时间
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeVO {
    // 字典主键
    #[serde(serialize_with = "u64_to_string")]
    pub id: u64,
    // 字典名称
    pub dict_name: Option<String>,
    // 字典类型
    pub dict_type: Option<String>,
    // 排序
    pub sort: Option<i32>,
    // 状态（0正常 1停用）
    pub status: Option<i8>,
    // 备注
    pub remark: Option<String>,
}

impl From<DictType> for DictTypeVO {
    fn from(req: DictType) -> Self {
        Self {
            id: req.id,
            dict_name: req.dict_name,
            dict_type: req.dict_type,
            sort: req.sort,
            status: req.status,
            remark: req.remark,
        }
    }
}