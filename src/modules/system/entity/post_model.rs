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
use crate::utils::string_utils::{serialize_option_u64_to_string,deserialize_string_to_u64,deserialize_string_to_i8};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostSaveRequest {
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
}

impl From<PostSaveRequest> for SystemPost {
    fn from(item: PostSaveRequest) -> Self {
        SystemPost {
            id: None,
            post_code: item.post_code,
            post_name: item.post_name,
            enabled: item.enabled,
            sort: item.sort,
            remark: item.remark,
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
}

impl From<PostUpdateRequest> for SystemPost {
    fn from(item: PostUpdateRequest) -> Self {
        SystemPost {
            id: item.id,
            post_code: item.post_code,
            post_name: item.post_name,
            enabled: item.enabled,
            sort: item.sort,
            remark: item.remark,
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
    pub post_name: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub enabled: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostPageBO {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub post_name: Option<String>,
    pub enabled: Option<i8>,
}

impl From<PostPageRequest> for PostPageBO {
    fn from(item: PostPageRequest) -> Self {
        PostPageBO {
            page_num: item.page_num,
            page_size: item.page_size,
            post_name: item.post_name,
            enabled: item.enabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostListVO {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id:  Option<u64>,
    /// 岗位编码，权限控制的时候使用
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    /// 岗位状态
    pub enabled: Option<i8>,
    /// 岗位排序
    pub sort:  Option<i32>,
    /// 备注
    pub remark: Option<String>,
    pub create_time:String,
}

impl From<SystemPost> for PostListVO {
    fn from(item: SystemPost) -> Self {
        PostListVO {
            id: item.id,
            post_code: item.post_code,
            post_name: item.post_name,
            enabled: item.enabled,
            sort: item.sort,
            remark: item.remark,
            create_time: item.create_time.unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}