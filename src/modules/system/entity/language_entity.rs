//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use serde::{Deserialize, Serialize};

/// 语言类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangType {
    // 自增ID
    pub id: Option<u64>,
    // 语言名称
    pub language_name: Option<String>,
    // 配置文件名称
    pub file_name: Option<String>,
    // 1启用0禁用
    pub status: Option<i8>,
    // 是否删除
    pub is_del: Option<i8>,
}

/// 浏览器语言类型表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangCountry {
    // 自增ID
    pub id: Option<u64>,
    // 管理语言类型
    pub type_id: Option<u64>,
    // 国家标识
    pub code: Option<String>,
    // 国家名称
    pub country_name: Option<String>,        
}

/// 语言code表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangCode {
    // 自增ID
    pub id: Option<u64>,
    // 语言类型
    pub type_id: Option<u64>,
    // 状态码
    pub code: Option<String>,
    // 备注说明
    pub remarks: Option<String>,
    // 说明
    pub lang_explain: Option<String>,
    // 服务端为 true，用户端为 false
    pub is_admin: Option<i8>,      
}

