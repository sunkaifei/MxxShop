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
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub admin_id: u64,
    pub role_ids: Vec<u64>,
    pub create_time: DateTime,
}