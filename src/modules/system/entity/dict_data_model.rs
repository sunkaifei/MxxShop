use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::modules::system::entity::dict_data_entity::DictData;
use crate::utils::string_utils::{string_to_u64};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DictDataSaveRequest {
    // 字典标签
    pub dict_label: Option<String>,
    // 字典键值
    pub dict_value: Option<String>,
    // 字典类型
    pub dict_type: Option<String>,
    // 字典排序
    pub dict_sort: Option<i32>,
    // 样式属性（其他样式扩展）
    pub css_class: Option<String>,
    // 表格回显样式
    pub list_class: Option<String>,
    // 是否默认（Y是 N否）
    pub is_default: Option<i8>,
    // 状态（0正常 1停用）
    pub status: Option<i8>,
    // 创建者
    pub create_by: Option<String>,
    // 更新者
    pub update_by: Option<String>,
    // 备注
    pub remark: Option<String>,
}

impl From<DictDataSaveRequest> for DictData {
    fn from(req: DictDataSaveRequest) -> Self {
        Self {
            id: 0,
            dict_sort: req.dict_sort,
            dict_label: req.dict_label,
            dict_value: req.dict_value,
            dict_type: req.dict_type,
            css_class: req.css_class,
            list_class: req.list_class,
            is_default: req.is_default,
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
pub struct DictDataUpdateRequest {
    // 字典编码
    #[serde(deserialize_with = "string_to_u64")]
    pub id: u64,
    // 字典标签
    pub dict_label: Option<String>,
    // 字典键值
    pub dict_value: Option<String>,
    // 字典类型
    pub dict_type: Option<String>,
    // 字典排序
    pub dict_sort: Option<i32>,
    // 样式属性（其他样式扩展）
    pub css_class: Option<String>,
    // 表格回显样式
    pub list_class: Option<String>,
    // 是否默认（Y是 N否）
    pub is_default: Option<i8>,
    // 状态（0正常 1停用）
    pub status: Option<i8>,
    // 创建者
    pub create_by: Option<String>,
    // 更新者
    pub update_by: Option<String>,
    // 备注
    pub remark: Option<String>,
}

impl From<DictDataUpdateRequest> for DictData {
    fn from(req: DictDataUpdateRequest) -> Self {
        Self {
            id: req.id,
            dict_sort: req.dict_sort,
            dict_label: req.dict_label,
            dict_value: req.dict_value,
            dict_type: req.dict_type,
            css_class: req.css_class,
            list_class: req.list_class,
            is_default: req.is_default,
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
pub struct DictDataPageRequest {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i32>,
    pub page_num: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryDictDataPageBO {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i32>,
    pub page_num: u64,
    pub page_size: u64,
}

impl From<DictDataPageRequest> for QueryDictDataPageBO {
    fn from(req: DictDataPageRequest) -> Self {
        Self {
            dict_name: req.dict_name,
            dict_type: req.dict_type,
            status: req.status,
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataQueryRequest {
    #[serde(rename = "dictType")]
    pub dict_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataValuesResponse {
    pub key: Option<String>,
    pub value: Option<String>,
    pub is_default: Option<i8>,
}