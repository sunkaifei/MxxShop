use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::modules::upload::entity::attach_entity::Attach;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadResult {
    ///图片地址
    pub src: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachUpdateRequest {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub upload_url: String,
    pub ext: String,
    pub size: u64,
    pub md5: String,
    pub r#type: i32,
    pub status: i32,
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
            add_time: DateTime::now(),
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttachPageRequest {
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    // 状态查询（0和空都是所有，1查询为0的数据，2查询为1的数据）
    pub config_type: Option<String>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachPageBO {
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub config_type: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}


impl From<AttachPageRequest> for AttachPageBO {
    fn from(req: AttachPageRequest) -> Self {
        Self {
            config_name: req.config_name,
            config_key: req.config_key,
            config_type: req.config_type.map(|s| {
                s.parse::<i8>().unwrap_or_else(|_| {
                    0
                })
            }),
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}

