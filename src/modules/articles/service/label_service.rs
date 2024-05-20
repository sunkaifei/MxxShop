//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use rbs::Value;

use crate::modules::articles::entity::label_entity::Label;
use crate::modules::articles::entity::label_model::{LabelSaveRequest, LabelUpdateRequest};
use crate::pool;
use crate::utils::snowflake_id;

pub async fn add_label(payload: LabelSaveRequest) -> Result<Value, Error> {
    let mut label_entity: Label = payload.into();
    match snowflake_id::generate_unique_id() {
        Ok(id) => {
            label_entity.id = id;
        }
        Err(_err) => {}
    }
    let rows = Label::insert(pool!(), &label_entity).await;
    Ok(rows?.last_insert_id)
}

pub async fn delete_label_by_ids(ids: Vec<String>) -> Result<ExecResult, Error> {
    let rows = Label::delete_in_column(pool!(), "id", &ids).await;
    return rows;
}

pub async fn update_label(payload: LabelUpdateRequest) -> Result<ExecResult, Error> {
    let label_entity: Label = payload.into();
    let rows = Label::update_by_column(pool!(), &label_entity, "id").await;
    return rows;
}

/*pub async fn get_label_by_id(id: u64) -> rbatis::Result<Option<Label>> {
    let st = Label::find_by_id(pool!(), id).await?
        .into_iter().next();
    Ok(st)
}*/