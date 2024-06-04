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
pub struct SystemPost {
    /// ID
    pub id: Option<u64>,
    /// 岗位编码，权限控制的时候使用
    pub post_code: Option<String>,
    /// 岗位名称
    pub post_name: Option<String>,
    /// 岗位状态
    pub enabled: Option<i8>,
    /// 岗位排序
    pub sort: Option<i32>,
    /// 备注
    pub remark: Option<String>,
    /// 创建日期
    pub create_time: Option<DateTime>,
    /// 更新时间
    pub update_time: Option<DateTime>,
    /// 是否删除
    pub is_del: Option<i8>,
}
