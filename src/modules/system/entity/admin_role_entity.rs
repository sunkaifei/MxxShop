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

///管理员与角色关联结构体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminRole {
    pub admin_id: u64,
    pub role_id: u64,
    pub create_time: Option<DateTime>,
}
