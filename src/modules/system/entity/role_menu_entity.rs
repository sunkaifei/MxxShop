use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleMenu {
    pub id: Option<u64>,
    pub parent_id: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemRoleMenu {
    pub id: Option<u64>,
    pub menu_id: Option<u64>,
    pub role_id: Option<u64>,
    pub status: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}
