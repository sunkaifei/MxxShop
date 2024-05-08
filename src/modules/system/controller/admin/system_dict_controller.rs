use std::collections::HashMap;

use actix_web::{delete, get, HttpResponse, post, put, web};
use actix_web_grants::protect;

use crate::core::web::entity::common::{BathIdRequest, InfoId};
use crate::core::web::response::{ResultPage, ResVO};
use crate::modules::system::entity::dict_data_model::{DataQueryRequest, DataValuesResponse, DictDataSaveRequest};
use crate::modules::system::entity::dict_type_model::{DictTypePageRequest, DictTypePageVO, DictTypeSaveRequest, DictTypeVO};
use crate::modules::system::service::{dict_service};

#[post("/system/dict/type/save")]
pub async fn save_dict_type(payload: web::Json<DictTypeSaveRequest>) -> HttpResponse {
    let type_request = payload.0;
    return match dict_service::save_dict_type(type_request).await {
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(v))
        }
        Err(e) => {
            log::error!("添加字典出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    };
}

#[post("/system/dict/data/save")]
pub async fn save_dict_data(payload: web::Json<DictDataSaveRequest>) -> HttpResponse {
    let data_request = payload.0;
    return match dict_service::save_dict_data(data_request).await {
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(v))
        }
        Err(e) => {
            log::error!("添加字典出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}

#[delete("/system/dict/type/batch_delete")]
pub async fn batch_delete_type(item: web::Json<BathIdRequest>) -> HttpResponse {
    if let Some(ids_vec) = item.ids.clone() {
        if ids_vec.is_empty() {
            HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()))
        } else {
            let result = dict_service::bath_delete_type(ids_vec).await;
            HttpResponse::Ok().json(&ResVO::<u64>::handle_result(result))
        }
    }else {
        HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()))
    }
}

#[delete("/system/dict/data/batch_delete")]
pub async fn batch_delete_data(item: web::Json<BathIdRequest>) -> HttpResponse {
    if let Some(ids_vec) = item.ids.clone() {
        let mut vec_id:Vec<Option<String>> = Vec::new();
        for string_id in ids_vec {
            if let Some(id) = string_id {
                if !id.is_empty() {
                    vec_id.push(Some(id));
                }
            }
        }
        let result = dict_service::bath_delete_data(vec_id).await;
        HttpResponse::Ok().json(&ResVO::<u64>::handle_result(result))
    }else {
        HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()))
    }
}

#[put("/system/dict/type/update")]
pub async fn update_dict_type(item: web::Json<DictTypeSaveRequest>) -> HttpResponse {
    if let Some(dict_name) = item.dict_name.clone() {
        if dict_name.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("字典名称不能为空".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("字典名称不能为空".to_string()));
    }
    return match dict_service::update_dict_type(item.0).await {
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(v))
        }
        Err(e) => {
            log::error!("更新字典出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    };
}

/// 更新字典数据
#[put("/system/dict/data/update")]
pub async fn update_dict_data(item: web::Json<DictDataSaveRequest>) -> HttpResponse {
    if let Some(dict_label) = item.dict_label.clone() {
        if dict_label.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("字典标签不能为空".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("字典标签不能为空".to_string()));
    }
    return match dict_service::update_dict_data(item.0).await{
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(v))
        }
        Err(e) => {
            log::error!("更新字典出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}

/// 获取字典类型详情
#[get("/system/dict/type/{id}")]
pub async fn get_dict_type_detail(item: web::Path<InfoId>) -> HttpResponse {
    //log::info!("==========1========{:?}", item.id.clone());
    if let Some(id) = item.id.clone() {
        if id.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("ID不能为空".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("ID不能为空".to_string()));
    }
    let id_string = item.into_inner().id.clone().unwrap_or_else(|| "0".to_string());
    let id_request: u64 = id_string.parse::<u64>().unwrap_or_else(|_| 0);
    return match dict_service::get_type_by_id(id_request).await {
        Ok(dict_type_op) => match dict_type_op {
            None => {
                HttpResponse::Ok().json(ResVO::<String>::error_msg("角色信息不存在".to_string()))
            }
            Some(dict_type) => {
                let dict_type_detail: DictTypeVO = dict_type.into();
                HttpResponse::Ok().json(ResVO::ok_with_data(dict_type_detail))
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

/// 获取字典数据详情
#[get("/system/dict/data/{id}")]
pub async fn get_dict_data_detail(item: web::Path<InfoId>) -> HttpResponse {
    if let Some(id) = item.id.clone() {
        if id.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("ID不能为空".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("ID不能为空".to_string()));
    }
    let id_string = item.into_inner().id.clone().unwrap_or_else(|| "0".to_string());
    let id_request: u64 = id_string.parse::<u64>().unwrap_or_else(|_| 0);
    return match dict_service::get_data_by_id(id_request).await {
        Ok(v) => {
            HttpResponse::Ok().json(ResVO::ok_with_data(v))
        }
        Err(e) => {
            log::error!("获取字典详情出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    };
}

#[get("/system/dict/type/list")]
#[protect("dict:type:list:show")]
pub async fn get_dict_type_page(item: web::Query<DictTypePageRequest>) -> HttpResponse {
    //log::info!("=================={:?}", item.0.clone());
    let page_result = dict_service::get_dict_type_page(item.0).await;
    return match page_result {
        Ok(page) => {
            let mut list_data: Vec<DictTypePageVO> = Vec::new();
            for data in page.records {
                list_data.push(DictTypePageVO {
                    id: data.id,
                    dict_name: data.dict_name,
                    dict_type: data.dict_type,
                    sort: data.sort,
                    status: data.status,
                    remark: data.remark,
                    update_time: data.update_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default(),
                })
            }
            let page_data = ResultPage {
                current_page: page.page_no,
                list: list_data,
                total: page.total,
            };
            HttpResponse::Ok().json(ResVO::ok_with_data(page_data))
        }
        Err(e) => {
            log::error!("查询字典出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}

#[get("/system/dict/data/list")]
#[protect("dict:data:list:show")]
pub async fn get_dict_data_list(path: web::Query<DataQueryRequest>) -> HttpResponse {
    let dict_type = &path.dict_type;
    let data_result = dict_service::get_dict_type_list(dict_type).await;
    return match data_result {
        Ok(v) => {
            let mut result_map: HashMap<String, Vec<DataValuesResponse>> = HashMap::new();
            let values: Vec<DataValuesResponse> = v.into_iter().map(|item| DataValuesResponse {
                key: item.dict_value,
                value: item.dict_label,
                is_default: item.is_default,
            }).collect();
            result_map.insert("values".to_string(), values);
            HttpResponse::Ok().json(ResVO::ok_with_data(result_map))
        }
        Err(e) => {
            log::error!("查询字典出错：{:}",e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}