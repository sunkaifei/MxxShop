use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attach {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub upload_url: String,
    pub ext: String,
    pub size: u64,
    pub md5: String,
    pub r#type: i32,
    pub status: i32,
    pub add_time: DateTime,
}

#[derive(Debug, MultipartForm)]
pub struct ImageForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
}

#[derive(Serialize)]
pub struct ImageData {
    pub id: u32,
    pub url: String,
}

#[derive(MultipartForm)]
pub struct AvatarForm {
    pub file: TempFile,
}