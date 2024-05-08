use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

///管理员与角色关联结构体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminRole {
    pub admin_id: u64,
    pub role_id: u64,
    pub create_time: Option<DateTime>,
}
