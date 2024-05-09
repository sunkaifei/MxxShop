//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::{fs, path};
use actix_identity::Identity;
use actix_multipart::{
    form::{
        MultipartForm,
    },
};
use actix_web::{HttpResponse, Error, HttpRequest};
use crate::core::web::response::ResVO;
use crate::modules::upload::entity::attach_entity::{AvatarForm, ImageForm};
use crate::modules::upload::service::attach_service;
use crate::modules::upload::service::attach_service::{upload_path, upload_url};
use crate::utils::{encryption_utils};

// 上传图片
pub async fn upload_image(req: HttpRequest, form: MultipartForm<ImageForm>, ) -> HttpResponse {
    let form = form.into_inner();

    return attach_service::upload_image("article".to_string(), form).await;
}


// 上传头像
pub async fn upload_avatar(user: Option<Identity>, form: MultipartForm<AvatarForm>, ) -> actix_web::Result<HttpResponse, Error> {
    if let Some(user) = user {
        let form = form.into_inner();

        let file_name = form.file.file_name.unwrap_or_else(|| "".to_string());

        if file_name.as_str() == "" {
            return Ok(HttpResponse::Ok().json(ResVO::<String>::error_msg("上传失败".to_string())));
        }

        let avatar_dir = upload_path("/avatar/".to_string(), "".to_string());

        if !path::Path::new(avatar_dir.as_str()).exists() {
            fs::create_dir_all(avatar_dir.as_str())?;
        }

        // let mut id: u64 = 0;
        // let user: UserLoginSession = serde_json::from_str(&user.id().unwrap_or_default()).unwrap();
        // id = user.id;

        let id: u64 = if let user_id = user.id()?.clone() {
            serde_json::from_str(&user_id).unwrap_or_default()
        } else {
            0
        };
        

        let name = encryption_utils::sha1(id.to_string().as_str());

        let ext = attach_service::get_extension(file_name.clone().as_str());
        let name = format!("{}.{}", name.clone(), ext);
        let path = upload_path("avatar".to_string(), name.clone());
        let url = upload_url("avatar".to_string(), name.clone());

        if let Err(err) = fs::copy(&form.file.file.path(), &path) {
            eprintln!("上传失败: {:?}", err);
            return Ok(HttpResponse::Ok().json(ResVO::<String>::error_msg("上传失败".to_string())));
        }

        Ok(HttpResponse::Ok().json(ResVO::<String>::ok_msg("上传成功".to_string())))
    } else {
        Ok(HttpResponse::Ok().json(ResVO::<String>::error("请登录".to_string(), 401)))
    }
}