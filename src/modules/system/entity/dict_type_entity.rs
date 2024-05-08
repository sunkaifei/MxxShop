use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DictType {
    // 字典主键
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
    // 创建时间
    pub create_time: Option<DateTime>,
    // 更新者
    pub update_by: Option<String>,
    // 更新时间
    pub update_time: Option<DateTime>,
    // 备注
    pub remark: Option<String>,
}
