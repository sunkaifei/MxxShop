use rbatis::rbdc::datetime::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub admin_id: u64,
    pub role_ids: Vec<u64>,
    pub create_time: DateTime,
}