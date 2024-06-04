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
use crate::modules::system::entity::navigation_entity::Navigation;
use crate::utils::string_utils::{serialize_option_u64_to_string,deserialize_string_to_u64,deserialize_string_to_i8};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavigationSaveRequest {
    /// 父id
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub parent_id: Option<u64>,
    /// 导航名称
    pub title_name: Option<String>,
    /// 自定义url地址
    pub url: Option<String>,
    /// 数据 id
    pub value: Option<i32>,
    /// 数据类型（custom:自定义导航, article_class:文章分类, customview:自定义页面）
    pub data_type: Option<String>,
    /// 导航类型（header:顶部导航, footer:底部导航）
    pub nav_type: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 是否显示（0否，1是）
    pub is_show: Option<i8>,
    /// 是否新窗口打开（0否，1是）
    pub is_new_window_open: Option<i8>,
}

impl From<NavigationSaveRequest> for Navigation {
    fn from(item: NavigationSaveRequest) -> Self {
        Navigation {
            id: None,
            parent_id: item.parent_id,
            title_name: item.title_name,
            url: item.url,
            value: item.value,
            data_type: item.data_type,
            nav_type: item.nav_type,
            sort: item.sort,
            is_show: item.is_show,
            is_new_window_open: item.is_new_window_open,
            create_time: Option::from(DateTime::now()),
            update_time: Option::from(DateTime::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavigationUpdateRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    /// 父id
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub parent_id: Option<u64>,
    /// 导航名称
    pub title_name: Option<String>,
    /// 自定义url地址
    pub url: Option<String>,
    /// 数据 id
    pub value: Option<i32>,
    /// 数据类型（custom:自定义导航, article_class:文章分类, customview:自定义页面）
    pub data_type: Option<String>,
    /// 导航类型（header:顶部导航, footer:底部导航）
    pub nav_type: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 是否显示（0否，1是）
    pub is_show: Option<i8>,
    /// 是否新窗口打开（0否，1是）
    pub is_new_window_open: Option<i8>,
}

impl From<NavigationUpdateRequest> for Navigation {
    fn from(item: NavigationUpdateRequest) -> Self {
        Navigation {
            id: item.id,
            parent_id: item.parent_id,
            title_name: item.title_name,
            url: item.url,
            value: item.value,
            data_type: item.data_type,
            nav_type: item.nav_type,
            sort: item.sort,
            is_show: item.is_show,
            is_new_window_open: item.is_new_window_open,
            create_time: Option::from(DateTime::now()),
            update_time: Option::from(DateTime::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavigationPageRequest {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    /// 导航名称
    pub title_name: Option<String>,
    /// 是否显示（0否，1是）
    pub is_show: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigationPageBO {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    /// 导航名称
    pub title_name: Option<String>,
    /// 是否显示（0否，1是）
    pub is_show: Option<i8>,
}

impl From<NavigationPageRequest> for NavigationPageBO {
    fn from(item: NavigationPageRequest) -> Self {
        NavigationPageBO {
            page_num: item.page_num,
            page_size: item.page_size,
            title_name: item.title_name,
            is_show: item.is_show,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavigationListVO {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id:  Option<u64>,
    /// 导航名称
    pub title_name: Option<String>,
    /// 数据类型（custom:自定义导航, article_class:文章分类, customview:自定义页面）
    pub data_type: Option<String>,
    /// 导航类型（header:顶部导航, footer:底部导航）
    pub nav_type: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 是否显示（0否，1是）
    pub is_show: Option<i8>,
    
    
}