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

use crate::modules::articles::entity::article_category_entity::ArticleCategory;
use crate::utils::string_utils::u64_to_string;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleCategorySaveRequest {
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
    //导航是否显示
    pub is_show: i8,
    //审核状态，0未审核，1已审核
    pub status: i8,
}

impl From<ArticleCategorySaveRequest> for ArticleCategory {
    fn from(arg: ArticleCategorySaveRequest) -> Self {
        Self {
            id: 0,
            parent_id: 0,
            short_url: arg.short_url,
            user_id: arg.user_id,
            category_name: arg.category_name,
            sort: arg.sort,
            count_topic: 0,
            create_time: Option::from(DateTime::now()),
            update_time: Option::from(DateTime::now()),
            is_show: arg.is_show,
            status: arg.status,
        }
    }
}

///文章分类树
#[derive(Debug, Serialize, Clone)]
pub struct CategoryTree {
    #[serde(serialize_with = "u64_to_string")]
    pub id: u64,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<CategoryTree>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryPageRequest {
    // 帖子分类名称
    pub category_name: Option<String>,
    //导航是否显示
    pub is_show: Option<String>,
    //审核状态，0未审核，1已审核
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryTreeVO {
    // 帖子分类ID
    #[serde(serialize_with = "u64_to_string")]
    pub id: u64,
    // 父分类ID
    #[serde(serialize_with = "u64_to_string")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<CategoryTreeVO>>,
}
