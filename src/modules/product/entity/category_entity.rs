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
pub struct ProductCategory {
    /// 自增id
    pub id: Option<u32>,
    /// 父id
    pub parent_id: Option<u32>,
    /// icon图标
    pub icon: Option<String>,
    /// 选中图标
    pub icon_active: Option<String>,
    /// 实景图
    pub realistic_images: Option<String>,
    /// 名称
    pub title_name: Option<String>,
    /// 副标题
    pub vice_name: Option<String>,
    /// 描述
    pub describe: Option<String>,
    /// css背景色值
    pub bg_color: Option<String>,
    /// 大图片
    pub big_images: Option<String>,
    /// 是否首页推荐（0否, 1是）
    pub is_home_recommended: Option<u8>,
    /// 排序
    pub sort: Option<u8>,
    /// 是否启用（0否，1是）
    pub is_enable: Option<u8>,
    /// SEO标题
    pub seo_title: Option<String>,
    /// SEO关键字
    pub seo_keywords: Option<String>,
    /// SEO描述
    pub seo_desc: Option<String>,
    /// 添加时间
    pub create_time: Option<DateTime>,
    /// 最后更新时间
    pub update_time: Option<DateTime>,
}
