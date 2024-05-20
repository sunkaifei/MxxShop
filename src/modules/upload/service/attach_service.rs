use std::ffi::OsStr;
use std::{fs, path};
use std::collections::HashMap;
use std::path::Path;
use actix_web::{HttpResponse};
use rbatis::{Page, PageRequest};
use rbatis::rbdc::{DateTime};
use crate::core::errors::error::Result;
use serde_json::json;
use crate::core::web::response::ResVO;
use crate::modules::upload::entity::attach_entity::{Attach, ImageForm};
use crate::pool;
use crate::utils::encryption_utils;
use crate::utils::settings::Settings;
use crate::utils::snowflake_id::generate_snowflake_id;
use crate::utils::time_utils::current_date;
use crate::modules::system::service::config_service;
use crate::modules::upload::entity::attach_model::AttachPageBO;

pub async fn upload_product_image(module: String, form: ImageForm) -> HttpResponse {
    let file_name = form.file.file_name.unwrap_or_else(|| "".to_string());

    if file_name.as_str() == "" {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("没有获取到文件，上传失败".to_string()));
    }

    let dir = upload_path(module.clone().to_string(), "".to_string());

    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap_or_else(|err| eprintln!("创建目录失败: {}", err));
    }

    let ext = get_extension(file_name.clone().as_str());
    if !is_image(ext.clone()) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("请上传正确图片类型".to_string()));
    }
    let name = format!("{}.{}", encryption_utils::uuid(), ext);
    let path = upload_path(module.clone().to_string(), name.clone());
    let url = upload_url(module.clone().to_string(), name.clone());

    let buffer = &fs::read(&form.file.file).expect("读取文件失败");
    let contents = String::from_utf8_lossy(&buffer).to_string();
    let md5 = encryption_utils::md5(contents.as_str());
    let size = buffer.len() as u64;

    
    
    let mut result_map: HashMap<String, String> = HashMap::new();
    let config_web = config_service::select_by_key(&"localStorage".to_string()).await;
    let local_storage = match config_web {
        Ok(config) => { 
            match config {  
                Some(config) => {
                    config.config_value.unwrap_or_default().to_string()
                },
                _ => {
                    return HttpResponse::Ok().json(ResVO::<String>::error_msg("获取上传服务器地址失败".to_string()));
                }
            }
        },
        Err(_) => {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("获取上传服务器地址失败".to_string()));
        }
    };
    
    // 判断是否有相同
    let attach_data = Attach::select_by_md5(pool!(), md5.as_str()).await;
    match attach_data {
        Ok(attach_option) => {
            match attach_option {
                Some(attach) => {
                    result_map.insert("fileName".to_string(), attach.name.to_string());
                    
                    let img_url = format!(
                        "{}{}",
                        &local_storage,
                        attach.upload_url.to_string()
                    ).to_string();
                    result_map.insert("src".to_string(), img_url);
                    return HttpResponse::Ok().json(ResVO::ok_with_data(result_map));
                }
                _ => {}
            }
        }
        Err(_) => {}
    };

    fs::copy(&form.file.file.path(), &path).unwrap_or_default();

    if !Path::new(&path).exists(){
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("上传失败".to_string()));
    }
    if cfg!(target_os = "linux") {
        let setting = Settings::get();
        //附件根目录路径
        let target_path = &setting.attach.upload_path;
        // 获取父级目录的权限
        let parent_permissions = match fs::metadata(&target_path) {
            Ok(metadata) => metadata.permissions(),
            Err(err) => {
                return HttpResponse::Ok().json(ResVO::<String>::error_msg(format!("linux下获取父级权限失败：{}",err)));
            }
        };
        // 设置新文件的权限
        fs::set_permissions(&path, parent_permissions).expect("父级目录权限设置给新文件失败");
    }
    
    let create_data = Attach {
        id: generate_snowflake_id(),
        name: file_name.clone(),
        path: path.clone(),
        upload_url: url.clone(),
        ext: ext.clone(),
        size,
        md5: md5.clone(),
        r#type: i32::from(2),
        status: i32::from(1),
        add_time: DateTime::now().into(),
    };
    let rows = Attach::insert(pool!(), &create_data.clone()).await;
    return if rows.unwrap_or_default().rows_affected > 0 {
        //result_map.insert("code".to_string(), "200".to_string());
        result_map.insert("fileName".to_string(), file_name.clone().to_string());
        let img_url = format!(
            "{}{}",
            &local_storage,
            &url.to_string()
        ).to_string();
        result_map.insert("src".to_string(), img_url);

        HttpResponse::Ok().json(ResVO::ok_with_data(result_map))
    } else {
        if fs::remove_file(&path).is_err() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("保存失败，图片删除错误".to_string()));
        }
        HttpResponse::Ok().json(ResVO::<String>::error_msg("上传数据保存失败".to_string()))
    }
}


pub async fn upload_image(module: String, form: ImageForm) -> HttpResponse {
    let file_name = form.file.file_name.unwrap_or_else(|| "".to_string());

    if file_name.as_str() == "" {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("没有获取到文件，上传失败".to_string()));
    }

    let dir = upload_path(module.clone().to_string(), "".to_string());

    if !path::Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.as_str()).unwrap_or_else(|err| eprintln!("创建目录失败: {}", err));
    }

    let ext = get_extension(file_name.clone().as_str());
    if !is_image(ext.clone()) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("请上传正确图片类型".to_string()));
    }
    let name = format!("{}.{}", encryption_utils::uuid(), ext);
    let path = upload_path(module.clone().to_string(), name.clone());
    let url = upload_url(module.clone().to_string(), name.clone());

    let buffer = &fs::read(&form.file.file).unwrap_or_default();
    let contents = String::from_utf8_lossy(&buffer).to_string();
    let md5 = encryption_utils::md5(contents.as_str());
    let size = buffer.len() as u64;

    let mut result_map: HashMap<String, String> = HashMap::new();

    // 判断是否有相同
    let attach_data = Attach::select_by_md5(pool!(), md5.as_str()).await;
    match attach_data {
        Ok(attach_option) => {
            match attach_option {
                Some(attach) => {
                    result_map.insert("code".to_string(), "200".to_string());
                    result_map.insert("src".to_string(), attach.upload_url.to_string());

                    let json = json!(result_map);

                    return HttpResponse::Ok()
                        .content_type("application/json")
                        .body(json.to_string());
                }
                _ => {}
            }
        }
        Err(_) => {}
    };

    if fs::copy(&form.file.file.path(), &path).is_err() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("上传失败".to_string()));
    }
    

    let create_data = Attach {
        id: generate_snowflake_id(),
        name: file_name.clone(),
        path: path.clone(),
        upload_url: url.clone(),
        ext: ext.clone(),
        size,
        md5: md5.clone(),
        r#type: i32::from(2),
        status: i32::from(1),
        add_time: DateTime::now().into(),
    };
    //let mut tx = RB.acquire_begin().await?;
    let rows = Attach::insert(pool!(), &create_data.clone()).await;
    //tx.commit().await?;
    //tx.rollback().await?;
    if rows.is_ok() {
        result_map.insert("code".to_string(), "200".to_string());
        result_map.insert("src".to_string(), url.to_string());

        let json = json!(result_map);

        return HttpResponse::Ok()
            .content_type("application/json")
            .body(json.to_string());
    } else {
        if fs::remove_file(&path).is_err() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("保存失败，图片删除错误".to_string()));
        }
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("上传数据保存失败".to_string()));
    }
}

///上传路径
///module 模块名称  如：article
///file_name 文件名称 如：123.jpg
pub fn upload_path(module: String, file_name: String) -> String {
    let setting = Settings::get();
    let path = &setting.attach.upload_path;
    let date_time = "/".to_string() + &current_date() + &"/".to_string();
    format!("{}{}{}{}", path, module, date_time, file_name)
}

pub fn upload_url(module: String, file_name: String) -> String {
    let setting = Settings::get();
    let path = &setting.attach.upload_url;
    let date_time = "/".to_string() + &current_date() + &"/".to_string();
    format!("{}{}{}{}", path, module, date_time, file_name)
}


pub fn get_extension(filename: &str) -> String {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);

    if let Some(ext) = extension {
        return ext.to_string();
    }

    "".to_string()
}

pub fn is_image(extension: String) -> bool {
    extension.eq("png")
        || extension.eq("jpg")
        || extension.eq("jpeg")
        || extension.eq("ico")
        || extension.eq("gif")
        || extension.eq("bmp")
        || extension.eq("svg")
}

///批量删除附件信息
pub async fn delete_in_column(ids: Vec<Option<String>>) -> Result<u64> {
    let result = Attach::delete_in_column(pool!(), "id", &ids).await;
    return Ok(result.unwrap_or_default().rows_affected);
}

///更新附件信息
pub async fn update_attach(item: &Attach) -> Result<u64> {
    let result = Attach::update_by_column(pool!(), item, "id").await;
    return Ok(result.unwrap_or_default().rows_affected); 
}

pub async fn select_attach_page(item: AttachPageBO) -> rbatis::Result<Page<Attach>> {
    let page_req = &PageRequest::new(item.page_num.clone(), item.page_size.clone());
    let result = Attach::select_attach_page(pool!(), page_req, item).await;
    Ok(result?)
}