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
use crate::modules::system::entity::language_entity::{LangCode, LangCountry, LangType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangTypeSaveRequest {
    // 语言名称
    pub language_name: Option<String>,
    // 配置文件名称
    pub file_name: Option<String>,
    // 1启用0禁用
    pub status: Option<i8>,
    // 是否删除
    pub is_del: Option<i8>,
}

impl From<LangTypeSaveRequest> for LangType {
    fn from(req: LangTypeSaveRequest) -> Self {
        Self {
            id: None,
            language_name: req.language_name,
            file_name: req.file_name,
            status: req.status,
            is_del: req.is_del,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangTypeUpdateRequest {
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

impl From<LangTypeUpdateRequest> for LangType {
    fn from(req: LangTypeUpdateRequest) -> Self {
        Self {
            id: req.id,
            language_name: req.language_name,
            file_name: req.file_name,
            status: req.status,
            is_del: req.is_del,
        }
    }
}


/// 语言类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryTypePageDTO {
    // 语言名称
    pub language_name: Option<String>,
    // 1启用0禁用
    pub status: Option<i8>,
    // 是否删除
    pub is_del: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangCountrySaveRequest {
    // 管理语言类型
    pub type_id: Option<u64>,
    // 国家标识
    pub code: Option<String>,
    // 国家名称
    pub country_name: Option<String>,
}

impl From<LangCountrySaveRequest> for LangCountry {
    fn from(req: LangCountrySaveRequest) -> Self {
        Self {
            id: None,
            type_id: req.type_id,
            code: req.code,
            country_name: req.country_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangCountryUpdateRequest {
    // 自增ID
    pub id: Option<u64>,
    // 管理语言类型
    pub type_id: Option<u64>,
    // 国家标识
    pub code: Option<String>,
    // 国家名称
    pub country_name: Option<String>,
}


impl From<LangCountryUpdateRequest> for LangCountry {
    fn from(req: LangCountryUpdateRequest) -> Self {
        Self {
            id: req.id,
            type_id: req.type_id,
            code: req.code,
            country_name: req.country_name,
        }
    }
}


/// 浏览器语言类型表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryCountryPageDTO {
    // 管理语言类型
    pub type_id: Option<u64>,
    // 国家标识
    pub code: Option<String>,
    // 国家名称
    pub country_name: Option<String>,        
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangCodeSaveRequest {
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

impl From<LangCodeSaveRequest> for LangCode {
    fn from(req: LangCodeSaveRequest) -> Self {
        Self {
            id: None,
            type_id: req.type_id,
            code: req.code,
            remarks: req.remarks,
            lang_explain: req.lang_explain,
            is_admin: req.is_admin,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangCodeUpdateRequest {
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

impl From<LangCodeUpdateRequest> for LangCode {
    fn from(req: LangCodeUpdateRequest) -> Self {
        Self {
            id: req.id,
            type_id: req.type_id,
            code: req.code,
            remarks: req.remarks,
            lang_explain: req.lang_explain,
            is_admin: req.is_admin,
        }
    }
}
/// 语言code表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryCodePageDTO {
    // 语言类型
    pub type_id: Option<u64>,
    // 状态码
    pub code: Option<String>,
    // 服务端为 true，用户端为 false
    pub is_admin: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}