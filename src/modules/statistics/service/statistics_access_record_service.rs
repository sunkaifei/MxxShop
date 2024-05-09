//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use crate::modules::statistics::entity::statistics_access_record_model::StatisticsAccessRecordDTO;
use crate::core::errors::error::Result;
use crate::modules::statistics::entity::statistics_access_record_entity::StatisticsAccessRecord;
use crate::RB;
use crate::utils::snowflake_id::generate_snowflake_id;

pub async fn save_statistics_record(record: StatisticsAccessRecordDTO) -> Result<u64> {
    let mut record_entity: StatisticsAccessRecord = record.into();
    record_entity.id = Option::from(generate_snowflake_id());
    let result = StatisticsAccessRecord::insert(&RB.clone(), &record_entity).await;
    return Ok(result.unwrap_or_default().rows_affected);
}