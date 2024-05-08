use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fans {
    ///ID
    pub id: u64,
    ///被关注者用户id
    pub follow_id: u64,
    ///粉丝id，关注者的id
    pub fans_id: u64,
    ///关注时间
    pub create_time: Option<DateTime>,
}