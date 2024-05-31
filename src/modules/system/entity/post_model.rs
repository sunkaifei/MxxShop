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
use crate::modules::system::entity::post_entity::SystemPost;
use crate::utils::string_utils::{serialize_option_u64_to_string,deserialize_string_to_u64};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostSaveRequest {
    /// 岗位名称
    pub name: Option<String>,
    /// 岗位状态
    pub enabled: Option<i8>,
    /// 岗位排序
    pub sort: Option<i32>,
}

impl From<PostSaveRequest> for SystemPost {
    fn from(item: PostSaveRequest) -> Self {
        SystemPost {
            id: None,
            name: item.name,
            enabled: item.enabled,
            sort: item.sort,
            create_time: Option::from(DateTime::now()),
            update_time: Option::from(DateTime::now()),
            is_del: Option::from(0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostUpdateRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    /// 岗位名称
    pub name: Option<String>,
    /// 岗位状态
    pub enabled: Option<i8>,
    /// 岗位排序
    pub sort: Option<i32>,
}

impl From<PostUpdateRequest> for SystemPost {
    fn from(item: PostUpdateRequest) -> Self {
        SystemPost {
            id: item.id,
            name: item.name,
            enabled: item.enabled,
            sort: item.sort,
            create_time: None,
            update_time: Option::from(DateTime::now()),
            is_del: None,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostPageRequest {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
    pub enabled: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostPageBO {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
    pub enabled: Option<i8>,
}

impl From<PostPageRequest> for PostPageBO {
    fn from(item: PostPageRequest) -> Self {
        PostPageBO {
            page_num: item.page_num,
            page_size: item.page_size,
            name: item.name,
            enabled: item.enabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostListVO {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id:  Option<u64>,
    pub name: Option<String>,
    /// 岗位状态
    pub enabled: Option<i8>,
    /// 岗位排序
    pub sort:  Option<i32>,
    pub create_time:String,
}