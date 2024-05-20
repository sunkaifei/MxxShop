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
pub struct SystemMenu {
    pub id: Option<u64>,
    /// 菜单名称
    pub menu_name: Option<String>,
    /// 父菜单ID
    pub parent_id: Option<u64>,
    /// 组件名称
    pub name: Option<String>,
    ///路由地址
    pub path: Option<String>,
    ///请求api地址
    pub api_url: Option<String>,
    /// 显示顺序
    pub sort: Option<i32>,
    ///组件路径
    pub component: Option<String>,
    ///是否为外链（0是 1否）
    pub is_link: Option<i8>,
    ///是否缓存（0缓存 1不缓存）
    pub is_keep_alive: Option<i8>,
    ///外链/内嵌时链接地址（http:xxx.com），开启外链条件，`1、isFrame:true 2、链接地址不为空`
    pub link_url: Option<String>,
    ///菜单高亮
    pub active: Option<String>,
    ///菜单类型（M目录 C菜单 F按钮）
    pub menu_type: Option<String>,
    ///是否隐藏（0显示 1隐藏）
    pub is_hide: Option<i8>,
    ///权限标识
    pub perms: Option<String>,
    ///菜单图标
    pub icon: Option<String>,
    ///是否固定，类似首页控制台在标签中是没有关闭按钮的
    pub is_affix: Option<i8>,
    ///颜色值
    pub color: Option<String>,
    ///是否内嵌，开启条件，`1、isIframe:true 2、链接地址不为空`
    pub is_iframe: Option<i8>,
    ///创建者
    pub create_by: Option<String>,
    ///创建时间
    pub create_time: Option<DateTime>,
    ///更新者
    pub update_by: Option<String>,
    ///更新时间
    pub update_time: Option<DateTime>,
    ///备注
    pub remark: Option<String>,
    ///状态（0正常 1停用）
    pub status: Option<i8>,
}
