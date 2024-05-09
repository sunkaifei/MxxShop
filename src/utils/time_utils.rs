//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::str::FromStr;

pub use chrono::{
    DateTime,
    NaiveDateTime,
    TimeZone,
    Utc,
};
use chrono::{Datelike, TimeDelta};
use chrono_tz::Tz;

use crate::utils::settings::Settings;

// 时区
pub fn timezone() -> Tz {
    let setting = Settings::get();
    let timezone = &setting.time.timezone;
    let tz = Tz::from_str(timezone).unwrap();
    tz
}

/// 获取当前日期
/// 格式：2020/01/01
pub fn current_date() -> String {
    // 获取当前本地时间
    let tz = timezone();

    let now = Utc::now().with_timezone(&tz);
    // 格式化时间为字符串
    let date_str = now.format("%Y/%m/%d").to_string();

    date_str
}

// 当前时间
pub fn now() -> DateTime<Tz> {
    let tz = timezone();

    Utc::now().with_timezone(&tz)
}

// 解析时间
pub fn parse(d: &str) -> DateTime<Tz> {
    let tz = timezone();

    let date = NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M:%S").unwrap_or_default();

    tz.from_utc_datetime(&date)
}

// 解析时间戳
pub fn from_timestamp(t: i64) -> DateTime<Tz> {
    let tz = timezone();
    let date = DateTime::from_timestamp(t, 0).unwrap_or_default();
    tz.from_utc_datetime(&date.naive_utc())
}


pub fn compare_with_current_time(db_timestamp: &str) -> String {
    let tz = timezone();
    let db_datetime = NaiveDateTime::parse_from_str(db_timestamp, "%Y-%m-%d %H:%M:%S").unwrap_or_default();
    let current_utc = Utc::now().with_timezone(&tz);
    let local_now = current_utc.naive_local();
    let duration = local_now.signed_duration_since(db_datetime);
    // 判断时间差是否小于 1 小时
    if duration < TimeDelta::try_hours(1).unwrap_or_default() {
        let minutes = duration.num_minutes();
        return format!("{}分钟前", minutes);
    } else if duration > TimeDelta::try_hours(1).unwrap_or_default() {
        let hours = duration.num_hours();
        return format!("{}小时前", hours);
    } else if duration > TimeDelta::try_days(30).unwrap_or_default() {
        let days = duration.num_days();
        return format!("{}天前", days);
    }
    let db_year = db_datetime.year();
    let now_year = local_now.year();
    if db_year == now_year {
        tz.from_utc_datetime(&db_datetime).format("%m-%d %H:%M").to_string()
    } else {
        tz.from_utc_datetime(&db_datetime).format("%Y-%m-%d %H:%M").to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::time_utils::compare_with_current_time;

    #[test]
    pub fn time() {
        let db_timestamp = "2023-12-18 02:26:00";
        let result = compare_with_current_time(db_timestamp);
        println!("{}", result); // 输出：02:19

        let db_timestamp = "2023-12-15 08:30:00";
        let result = compare_with_current_time(db_timestamp);
        println!("{}", result); // 输出：2天7时49分

        let db_timestamp = "2023-11-15 13:45:00";
        let result = compare_with_current_time(db_timestamp);
        println!("{}", result); // 输出：2023-11-15 13:45

        let db_timestamp = "2022-11-15 13:45:00";
        let result = compare_with_current_time(db_timestamp);
        println!("{}", result); // 输出：2023-11-15 13:45
    }
}