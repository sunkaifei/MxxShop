use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemDept {
    pub id: Option<u64>,
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
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
}
