//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteLinks {
    /// id
    pub id: Option<u64>,
    /// 有钱链接类型：0文字链接，1logo链接
    pub link_type: Option<i32>,
    /// 网站名称
    pub link_name: Option<String>,
    /// 网站地址
    pub link_url: Option<String>,
    /// 网站logo地址
    pub link_logo: Option<String>,
    /// 是否显示，0不显示，1显示
    pub status: Option<i32>,
    /// 排序
    pub sort: Option<i32>,
    /// 添加时间
    pub create_time: Option<DateTime>,
    /// 0删除，1显示
    pub deleted: Option<i8>,
}