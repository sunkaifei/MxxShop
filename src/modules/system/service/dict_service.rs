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
use rbatis::rbdc::db::ExecResult;
use crate::core::errors::error::Result;

use crate::modules::system::entity::dict_data_entity::DictData;
use crate::modules::system::entity::dict_data_model::{DictDataSaveRequest, DictDataPageRequest, QueryDictDataPageBO};
use crate::modules::system::entity::dict_type_entity::DictType;
use crate::modules::system::entity::dict_type_model::{DictTypePageBO, DictTypePageRequest, DictTypeSaveRequest};
use crate::pool;
use crate::utils::snowflake_id::generate_snowflake_id;

//添加字典类型数据
pub async fn save_dict_type(payload: DictTypeSaveRequest) -> Result<ExecResult> {
    let mut data_req: DictType = payload.into();
    data_req.id = generate_snowflake_id();
    Ok(DictType::insert(pool!(), &data_req).await?)
}

//添加字典数据
pub async fn save_dict_data(payload: DictDataSaveRequest) -> Result<ExecResult> {
    let mut data_req: DictData = payload.into();
    data_req.id = generate_snowflake_id();
    Ok(DictData::insert(pool!(), &data_req).await?)
}

///批量删除字典类型
pub async fn bath_delete_type(ids: Vec<Option<String>>) -> Result<u64> {
    let result = DictType::delete_in_column(pool!(), "id", &ids).await;
    return Ok(result?.rows_affected);
}

///批量删除字典数据
pub async fn bath_delete_data(ids: Vec<Option<String>>) -> Result<u64> {
    let result = DictData::delete_in_column(pool!(), "id", &ids).await;
    return Ok(result?.rows_affected);
}

///按id更新系统字典类型内容
pub async fn update_dict_type(item: DictTypeSaveRequest) -> Result<ExecResult> {
    let data_req: DictType = item.into();
    Ok(DictType::update_by_column(pool!(), &data_req,"id").await?)
}

///按id更新系统字典数据内容
pub async fn update_dict_data(item: DictDataSaveRequest) -> Result<ExecResult> {
    let data_req: DictData = item.into();
    Ok(DictData::update_by_column(pool!(), &data_req, "id").await?)
}

///按id查询系统字典类型内容
pub async fn get_type_by_id(id: &Option<u64>) -> rbatis::Result<Option<DictType>> {
    let st = DictType::select_by_column(pool!(), "id", id).await?
        .into_iter()
        .next();
    Ok(st)
}

///按id系统字典数据内容
pub async fn get_data_by_id(id: &Option<u64>) -> rbatis::Result<Option<DictData>> {
    let st = DictData::select_by_column(pool!(), "id", id).await?
        .into_iter()
        .next();
    Ok(st)
}

///获取字典数据列表
pub async fn get_dict_data_list(item: DictDataPageRequest) -> rbatis::Result<Page<DictData>> {
    let page_query:QueryDictDataPageBO = item.into();
    let page_req = &PageRequest::new(page_query.page_num, page_query.page_size.clone());
    let list = DictData::select_page(pool!(), page_req, &page_query).await?;
    return Ok(list);
}

///获取字典类型列表翻页
pub async fn get_dict_type_page(item: DictTypePageRequest) -> rbatis::Result<Page<DictType>> {
    let page_query: DictTypePageBO = item.into();
    let page_req = &PageRequest::new(page_query.page_num.clone(), page_query.page_size.clone());
    let list = DictType::select_page(pool!(), page_req, &page_query).await?;
    return Ok(list);
}

///按字典类型获取字典数据列表
pub async fn get_dict_type_list(dict_type: &String) -> rbatis::Result<Vec<DictData>> {
    let list = DictData::select_by_column(pool!(), "dict_type", dict_type).await;
    return list;
}