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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemRole {
    /// 角色ID
    pub id: Option<u64>,
    /// 角色名称
    pub role_name: Option<String>,
    /// 角色权限字符串
    pub role_key: Option<String>,
    /// 显示顺序
    pub sort: Option<i32>,
    /// 数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub data_scope: Option<i8>,
    /// 角色状态（0正常 1停用）
    pub status: Option<i8>,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: Option<i8>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<DateTime>,
    /// 备注
    pub remark: Option<String>,
}

