use rbatis::crud;
use crate::modules::statistics::entity::statistics_access_record_entity::StatisticsAccessRecord;


crud!(StatisticsAccessRecord{}, "fly_statistics_access_record");