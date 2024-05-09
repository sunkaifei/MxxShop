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
pub struct DictData {
    // 字典编码
    pub id: u64,
    // 字典排序
    pub dict_sort: Option<i32>,
    // 字典标签
    pub dict_label: Option<String>,
    // 字典键值
    pub dict_value: Option<String>,
    // 字典类型
    pub dict_type: Option<String>,
    // 样式属性（其他样式扩展）
    pub css_class: Option<String>,
    // 表格回显样式
    pub list_class: Option<String>,
    // 是否默认（Y是 N否）
    pub is_default: Option<i8>,
    // 状态（0正常 1停用）
    pub status: Option<i8>,
    // 创建者
    pub create_by: Option<String>,
    // 创建时间
    pub create_time: Option<DateTime>,
    // 更新者
    pub update_by: Option<String>,
    // 更新时间
    pub update_time: Option<DateTime>,
    // 备注
    pub remark: Option<String>,
}
