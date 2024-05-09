//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

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
