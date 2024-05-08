use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::modules::system::entity::dept_entity::SystemDept;
use crate::utils::string_utils::{deserialize_string_to_u64,serialize_option_u64_to_string};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DeptSaveRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub parent_id: Option<u64>,
    pub dept_name: Option<String>,
    pub sort: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<i8>,
}

impl From<DeptSaveRequest> for SystemDept {
    fn from(s: DeptSaveRequest) -> Self {
        Self {
            id: Option::from(0),
            parent_id: s.parent_id,
            ancestors: Option::from(String::from("")),
            dept_name: Option::from(s.dept_name),
            sort: s.sort,
            leader: Option::from(String::from("")),
            phone: s.phone,
            email: s.email,
            status: s.status,
            del_flag: Option::from(0),
            create_by: Option::from(String::from("")),
            create_time: Option::from(DateTime::now()),
            update_by: Option::from(String::from("")),
            update_time: Option::from(DateTime::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DeptUpdateRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub parent_id: Option<u64>,
    pub dept_name: Option<String>,
    pub sort: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<i8>,
}

impl From<DeptUpdateRequest> for SystemDept {
    fn from(s: DeptUpdateRequest) -> Self {
        Self {
            id: s.id,
            parent_id: s.parent_id,
            ancestors: Option::from(String::from("")),
            dept_name: s.dept_name,
            sort: s.sort,
            leader: s.leader,
            phone: s.phone,
            email: s.email,
            status: s.status,
            del_flag: Option::from(0),
            create_by: Option::from(String::from("")),
            create_time: Option::from(DateTime::now()),
            update_by: Option::from(String::from("")),
            update_time: Option::from(DateTime::now()),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DeptPageRequest {
    pub dept_name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeptPageBO {
    pub dept_name: Option<String>,
    pub status: Option<i8>,
}

impl From<DeptPageRequest> for DeptPageBO {
    fn from(s: DeptPageRequest) -> Self {
        Self {
            dept_name: s.dept_name,
            status: Option::from(s.status.and_then(|s| s.parse().ok()).map_or_else(|| 0i8, |i| i)),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct DeptTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DeptTree>>,
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    pub label: String,
    pub is_disabled: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct DeptTreeData {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub parent_id: Option<u64>,
    pub dept_name: Option<String>,
    pub sort: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<i8>,
    pub update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DeptTreeData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct DeptListData {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub parent_id: Option<u64>,
    pub dept_name: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i8>,
    pub update_time: Option<String>,
}

impl From<SystemDept> for DeptListData {
    fn from(s: SystemDept) -> Self {
        Self {
            id: s.id,
            parent_id: s.parent_id,
            dept_name: Option::from(s.dept_name),
            sort: s.sort,
            status: s.status,
            update_time: s.update_time.map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SystemDeptVO {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub parent_id: Option<u64>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub sort: Option<i32>,
    ///负责人
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<i8>,
    pub del_flag: Option<i8>,
    pub create_by: Option<String>,
    pub create_time: Option<String>,
    pub update_by: Option<String>,
    pub update_time: Option<String>,
}

impl From<SystemDept> for SystemDeptVO {
    fn from(s: SystemDept) -> Self {
        Self {
            id: s.id,
            parent_id: s.parent_id,
            ancestors: s.ancestors,
            dept_name: s.dept_name,
            sort: s.sort,
            leader: s.leader,
            phone: s.phone,
            email: s.email,
            status: s.status,
            del_flag: s.del_flag,
            create_by: s.create_by,
            create_time: Option::from(s.create_time.map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default()),
            update_by: s.update_by,
            update_time: Option::from(s.update_time.map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default()),
        }
    }
}