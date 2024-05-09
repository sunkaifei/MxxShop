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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleCategory {
    // 帖子分类ID
    pub id: u64,
    // 父分类ID
    pub parent_id: u64,
    // 短网址
    pub short_url: Option<String>,
    // 用户ID
    pub user_id: Option<u64>,
    // 帖子分类名称
    pub category_name: String,
    // 排序
    pub sort: i32,
    // 统计帖子
    pub count_topic: i32,
    // 添加时间
    pub create_time: Option<DateTime>,
    // 更新时间
    pub update_time: Option<DateTime>,
    // 导航是否显示
    pub is_show: i8,
    // 审核状态，0未审核，1已审核
    pub status: i8,
}