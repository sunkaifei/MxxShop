//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use regex::Regex;
use serde::{Deserialize, Deserializer, Serializer};

/// 字符串中汉字的个数
pub fn chinese_string_count(chinese_count: String) -> i32 {
    let mut count = 0;
    for c in chinese_count.chars() {
        if is_chinese_character(c) {
            count += 1;
        }
    }
    return count;
}

/// 判断一个字符是否为有效的汉字
fn is_chinese_character(c: char) -> bool {
    ('\u{4E00}' <= c && c <= '\u{9FFF}') || ('\u{3400}' <= c && c <= '\u{4DBF}')
}

///i8转为字符
pub fn i8_to_string<S>(value: &i8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

///字符转为i8
pub fn string_to_i8<'de, D>(deserializer: D) -> Result<i8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i8>().map_err(serde::de::Error::custom)
}

///u64转为字符
pub fn u64_to_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

///字符转为u64
pub fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}

///字符串转为<Option<i8>
pub fn deserialize_string_to_i8<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
    where
        D: Deserializer<'de>,
{
    let opt_string: Option<String> = Option::deserialize(deserializer)?;
    let opt_i8: Option<i8> = match opt_string {
        Some(s) => Some(s.parse().unwrap_or_default()),
        None => None,
    };
    Ok(opt_i8)
}

pub fn serialize_option_i8_to_string<S>(
    value: &Option<i8>,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    match value {
        Some(val) => serializer.serialize_str(&val.to_string()),
        None => serializer.serialize_none(),
    }
}

///字符串转为<Option<u64>
pub fn deserialize_string_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_string: Option<String> = Option::deserialize(deserializer)?;
    let opt_u64: Option<u64> = match opt_string {
        Some(s) => Some(s.parse().unwrap_or_default()),
        None => None,
    };
    Ok(opt_u64)
}

pub fn serialize_option_u64_to_string<S>(
    value: &Option<u64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(val) => serializer.serialize_str(&val.to_string()),
        None => serializer.serialize_none(),
    }
}

///获取第一个汉字
pub fn get_first_chinese_character(s: &str) -> Option<char> {
    for c in s.chars() {
        if is_chinese_character(c) {
            return Some(c);
        }
    }
    None
}

///判断字符串不能有除汉字以外的字符，字符串数必须大于0小于2
pub fn is_valid_string(s: &str) -> bool {
    let re = Regex::new(r"^[\u{4e00}-\u{9fff}]{1,2}$").unwrap();
    re.is_match(s)
}

///数字的五行分类
///按照数理进行五行分类：尾数为1，2五行为木；尾数为3，4五行为火；尾数为5，6五行为土；尾数为7，8五行为金；尾数为9，0五行为水；
pub fn wuxing_classification(num: i32) -> &'static str {
    let last_digit = num % 10;
    match last_digit {
        1 | 2 => "木",
        3 | 4 => "火",
        5 | 6 => "土",
        7 | 8 => "金",
        9 | 0 => "水",
        _ => "未知五行", // 处理其他情况，例如超出范围的数字
    }
}

// 这个函数将被调用来序列化 Option<String> 类型的值。
pub fn serialize_option_string<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    match value {
        Some(s) => serializer.serialize_str(s),
        None => serializer.serialize_str(""),
    }
}

///分析客户端操作系统
pub fn user_agent_os(user_agent: &str) -> Option<&str> {
    // 使用模式匹配来检查包含的操作系统信息
    match user_agent {
        ua if ua.contains("Windows NT 10.0") => Some("Windows 10"),
        ua if ua.contains("Windows NT 6.2") => Some("Windows 8"),
        ua if ua.contains("Windows NT 6.1") => Some("Windows 7"),
        ua if ua.contains("Windows") => Some("Windows"),
        ua if ua.contains("iPhone OS 16") => Some("iOS 16"),
        ua if ua.contains("iPhone OS 15") => Some("iOS 15"),
        ua if ua.contains("iPhone OS 14") => Some("iOS 14"),
        ua if ua.contains("iPhone OS 13") => Some("iOS 13"),
        ua if ua.contains("iPhone OS 12") => Some("iOS 12"),
        ua if ua.contains("iPhone OS 11") => Some("iOS 11"),
        ua if ua.contains("iPhone OS 10") => Some("iOS 10"),
        ua if ua.contains("Android 14") => Some("Android 14"),
        ua if ua.contains("Android 13") => Some("Android 13"),
        ua if ua.contains("Android 12") => Some("Android 12"),
        ua if ua.contains("Android 11") => Some("Android 11"),
        ua if ua.contains("Android 10") => Some("Android 10"),
        ua if ua.contains("Android 9") => Some("Android 9"),
        ua if ua.contains("Android 8") => Some("Android 8"),
        ua if ua.contains("Mac") || ua.contains("macOS") => Some("macOS"),
        ua if ua.contains("Linux") => Some("Linux"),
        _ => Some("OthersOS"),
    }
}

///获取浏览器和版本号
pub fn user_agent_browser(user_agent: &str) -> Option<String> {
    let re = Regex::new(r#".*(Edg|Safari|Chrome|Firefox)/(\d+(\.\d+)+)$"#).unwrap();

    if let Some(captures) = re.captures(user_agent) {
        let browser = captures.get(1).unwrap().as_str();
        let version = captures.get(2).unwrap().as_str();
        Some(format!("{} {}", browser, version))
    } else {
        Some("Others".to_string())
    }
}

///将下划线转换为点
/// 例如："hello_world" => "hello.world"
pub fn convert_to_dot_notation(input: Option<&str>) -> Option<String> {
    if let Some(value) = input {
        let result = value.replace("_", ".");
        return Some(result);
    } else {
        None
    }
}

