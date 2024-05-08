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