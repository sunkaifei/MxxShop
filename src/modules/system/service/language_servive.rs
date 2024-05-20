//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use rbatis::{Page, PageRequest};
use crate::core::errors::error::Result;
use crate::modules::system::entity::language_entity::{LangCode, LangCountry, LangType};
use crate::modules::system::entity::language_model::{LangCodeSaveRequest, LangCodeUpdateRequest, LangCountrySaveRequest, LangCountryUpdateRequest, LangTypeSaveRequest, LangTypeUpdateRequest, QueryCodePageDTO, QueryCountryPageDTO, QueryTypePageDTO};
use crate::pool;

pub async fn save_lang_type(item: LangTypeSaveRequest) -> Result<u64> {
    let result = LangType::insert(pool!(), &item.into()).await;
    Ok(result?.rows_affected)
}

pub async fn bath_delete_type_by_id(ids: Vec<String>) -> Result<u64> {
    let result = LangType::delete_in_column(pool!(), "id", &ids).await;
    Ok(result.unwrap_or_default().rows_affected)
}

pub async fn update_type(item: LangTypeUpdateRequest) -> Result<u64> {
    let sys_five: LangType = item.into();
    let result = LangType::update_by_column(pool!(), &sys_five, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

/// 按id获取语言类型
pub async fn get_type_by_id(id: Option<u64>) -> rbatis::Result<Option<LangType>> {
    let st = LangType::select_by_column(pool!(), "id", id).await?
        .into_iter()
        .next();
    Ok(st)
}

pub async fn select_type_page(item: QueryTypePageDTO) -> rbatis::Result<Page<LangType>> {
    let page_req = &PageRequest::new(item.page_num.clone(), item.page_size.clone());
    let result = LangType::select_page(pool!(),page_req, &item).await;
    Ok(result?)
}


pub async fn save_country_type(item: LangCountrySaveRequest) -> Result<u64> {
    let result = LangCountry::insert(pool!(), &item.into()).await;
    Ok(result?.rows_affected)
}

pub async fn bath_delete_country_by_id(ids: Vec<String>) -> Result<u64> {
    let result = LangCountry::delete_in_column(pool!(), "id", &ids).await;
    Ok(result.unwrap_or_default().rows_affected)
}

pub async fn update_country(item: LangCountryUpdateRequest) -> Result<u64> {
    let sys_five: LangCountry = item.into();
    let result = LangCountry::update_by_column(pool!(), &sys_five, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

/// 按id获取语言类型
pub async fn get_country_by_id(id: Option<u64>) -> rbatis::Result<Option<LangCountry>> {
    let st = LangCountry::select_by_column(pool!(), "id", id).await?
        .into_iter()
        .next();
    Ok(st)
}

pub async fn select_country_page(item: QueryCountryPageDTO) -> rbatis::Result<Page<LangCountry>> {
    let page_req = &PageRequest::new(item.page_num.clone(), item.page_size.clone());
    let result = LangCountry::select_page(pool!(),page_req, &item).await;
    Ok(result?)
}



pub async fn save_code_type(item: LangCodeSaveRequest) -> Result<u64> {
    let result = LangCode::insert(pool!(), &item.into()).await;
    Ok(result?.rows_affected)
}


pub async fn bath_delete_code_by_id(ids: Vec<String>) -> Result<u64> {
    let result = LangCode::delete_in_column(pool!(), "id", &ids).await;
    Ok(result.unwrap_or_default().rows_affected)
}

pub async fn update_code(item: LangCodeUpdateRequest) -> Result<u64> {
    let sys_five: LangCode = item.into();
    let result = LangCode::update_by_column(pool!(), &sys_five, "id").await;
    return Ok(result.unwrap_or_default().rows_affected);
}

/// 
pub async fn get_code_by_id(id: Option<u64>) -> rbatis::Result<Option<LangCode>> {
    let st = LangCode::select_by_column(pool!(), "id", id).await?
        .into_iter()
        .next();
    Ok(st)
}

pub async fn select_code_page(item: QueryCodePageDTO) -> rbatis::Result<Page<LangCode>> {
    let page_req = &PageRequest::new(item.page_num.clone(), item.page_size.clone());
    let result = LangCode::select_page(pool!(),page_req, &item).await;
    Ok(result?)
}