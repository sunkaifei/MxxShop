use serde::{Deserialize, Serialize};
use crate::modules::system::entity::admin_entity::SystemAdmin;

use crate::modules::system::entity::role_entity::SystemRole;
use crate::utils::string_utils::{deserialize_string_to_u64,serialize_option_u64_to_string};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RoleSaveRequest {
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub menu_ids: Option<Vec<Option<String>>>,
    /// 显示顺序
    pub sort: Option<i32>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

impl From<RoleSaveRequest> for RoleDTO {
    fn from(s: RoleSaveRequest) -> Self {
        Self {
            id: None,
            role_name: s.role_name,
            role_key: s.role_key,
            sort: s.sort,
            status: s.status,
            remark: s.remark,
            menu_ids: s.menu_ids,
            admin: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RoleUpdateRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    pub role_name: Option<String>,
    /// 角色权限字符串
    pub role_key: Option<String>,
    pub menu_ids: Option<Vec<Option<String>>>,
    pub remark: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i8>,
}

impl From<RoleUpdateRequest> for RoleDTO {
    fn from(s: RoleUpdateRequest) -> Self {
        Self {
            id: s.id,
            role_name: s.role_name,
            role_key: s.role_key,
            sort: s.sort,
            status: s.status,
            remark: s.remark,
            menu_ids: s.menu_ids,
            admin: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleDTO {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    pub role_name: Option<String>,
    /// 角色权限字符串
    pub role_key: Option<String>,
    pub menu_ids: Option<Vec<Option<String>>>,
    pub remark: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i8>,
    pub admin: SystemAdmin,
}

impl SystemRole {
    pub fn process_role_dto(&mut self, dto: &RoleDTO) {
        self.id = dto.id.clone();
        self.role_name = dto.role_name.clone();
        self.role_key = dto.role_key.clone();
        self.remark = dto.remark.clone();
        self.sort = dto.sort.clone();
        self.status = dto.status.clone();
        self.create_by = dto.admin.user_name.clone();
        self.update_by = dto.admin.user_name.clone();
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RoleDetail {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    pub role_name: Option<String>,
    /// 角色权限字符串
    pub role_key: Option<String>,
    pub remark: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i8>,
}

impl From<SystemRole> for RoleDetail {
    fn from(s: SystemRole) -> Self {
        Self {
            id: s.id,
            role_name: s.role_name,
            role_key: s.role_key,
            sort: s.sort,
            status: s.status,
            remark: s.remark,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoleDetailVO {
    pub menu_ids: Vec<Option<String>>,
    pub role: RoleDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleMenuData {
    pub role_menus: Vec<u64>,
    pub menu_list: Vec<MenuDataList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDataList {
    pub id: u64,
    pub menu_name: String,
    pub parent_id: u64,
    pub name: String,
    pub path: String,
    pub sort: i32,
    pub component: Option<String>,
    pub menu_type: String,
    pub remark: String,
    pub status: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RoleListRequest {
    #[serde(rename = "pageNum")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    #[serde(rename = "roleName")]
    pub role_name: Option<String>,
    #[serde(rename = "role_status")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RoleListData {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    pub sort: Option<i32>,
    pub status: Option<i8>,
    pub role_name: Option<String>,
    pub remark: Option<String>,
    pub update_time: Option<String>,
}
