//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!



use std::collections::HashMap;
use serde_json::Value;


/// 模板国际化元素标签
/// 模板调用示例：{{ lang(info="system_site_login",locale="zh-CN",name="admin") }}
/// 你好，%{name}
pub fn lang_function(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let info = args.get("info").ok_or_else(|| tera::Error::msg("lang标签缺少info参数".to_string()))?;
    let info_str = info.as_str().unwrap_or_default();
    let tags = if let Some(locale) = args.get("locale").and_then(|l| l.as_str()) {
        if let Some(name) = args.get("name").and_then(|n| n.as_str()) {
            t!(info_str, locale = locale, name = name)
        } else {
            t!(info_str, locale = locale)
        }
    } else {
        t!(info_str)
    };

    Ok(Value::String(tags.to_string()))
}


