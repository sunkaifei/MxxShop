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
use crate::modules::upload::entity::attach_entity::Attach;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadResult {
    ///图片地址
    pub src: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachUpdateRequest {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub upload_url: Option<String>,
    pub ext: Option<String>,
    pub size: Option<u64>,
    pub md5: Option<String>,
    pub r#type: Option<i8>,
    pub status: Option<i8>,
}

impl From<AttachUpdateRequest> for Attach {
    fn from(req: AttachUpdateRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            path: req.path,
            upload_url: req.upload_url,
            ext: req.ext,
            size: req.size,
            md5: req.md5,
            r#type: req.r#type,
            status: req.status,
            add_time: Option::from(DateTime::now()),
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttachPageRequest {
    pub name: Option<String>,
    // 状态查询（0和空都是所有，1查询为0的数据，2查询为1的数据）
    pub r#type: Option<i8>,
    // 当前页码数
    pub page_num: Option<u64>,
    // 每页条数
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachPageBO {
    pub name: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub r#type: Option<i8>,
    // 当前页码数
    pub page_num: Option<u64>,
    // 每页条数
    pub page_size: Option<u64>,
}


impl From<AttachPageRequest> for AttachPageBO {
    fn from(req: AttachPageRequest) -> Self {
        Self {
            name: req.name,
            r#type: req.r#type,
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachPageVO {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub upload_url: Option<String>,
    pub ext: Option<String>,
    pub size: Option<u64>,
    pub md5: Option<String>,
    pub r#type: Option<i8>,
    pub status: Option<i8>,
    pub add_time: Option<String>,
}