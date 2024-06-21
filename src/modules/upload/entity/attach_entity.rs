//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attach {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub upload_url: Option<String>,
    pub ext: Option<String>,
    pub size: Option<u64>,
    pub md5: Option<String>,
    pub r#type: Option<i8>,
    pub status: Option<i8>,
    pub add_time: Option<DateTime>,
}

#[derive(Debug, MultipartForm)]
pub struct ImageForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
}

#[derive(Serialize)]
pub struct ImageData {
    pub id: Option<u64>,
    pub url: Option<String>,
}

#[derive(MultipartForm)]
pub struct AvatarForm {
    pub file: TempFile,
}