//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::time::SystemTime;

use chrono::{prelude::*};

/// get current time stamp
#[inline]
pub fn timestamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => { n.as_secs() }
        Err(_) => { 0 }
    }
}

/// 要求输入: 2019-11-11 10:10:10
#[inline]
pub fn from_str(datetime_str: &str) -> DateTime<Local> {
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse datetime");
    Local.from_local_datetime(&datetime).unwrap()
}

/// 当前的时间字符串
#[inline]
pub fn to_string() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 格式化时间
#[inline]
pub fn format(format_str: &str) -> String {
    let local: DateTime<Local> = Local::now();
    local.format(format_str).to_string()
}

/// 得到当前的日期
#[inline]
pub fn now() -> DateTime<Local> {
    Local::now()
}

/// 得到时分秒
#[inline]
pub fn time() -> (u32, u32, u32) {
    let now = now();
    (now.hour(), now.minute(), now.second())
}

/// 得到年月日
#[inline]
pub fn date() -> (u32, u32, u32) {
    let now = now();
    (now.year() as u32, now.month() as u32, now.day() as u32)
}

// 将时间戳转化为日期时间格式
/*#[inline]
pub fn datetime(timestamp: i64) -> String { 
    let dt = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}*/

/*fn format_time(time: SystemTime) -> String {
    let since_epoch = time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let formatted = since_epoch.as_secs().strftime("%Y-%m-%d %H:%M:%S").unwrap().to_string();

    formatted
}
*/