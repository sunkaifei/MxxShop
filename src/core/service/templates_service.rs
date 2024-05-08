use std::collections::HashMap;

use serde_json::{to_value};
use tera::{Error, Result, Tera, try_get_value, Value};
use crate::utils::time_utils::compare_with_current_time;

pub fn get_templates() -> Tera {
    let mut tera = Tera::new("./templates/**/*").unwrap_or_default();
    tera.autoescape_on(vec!["html"]);
    // 注册自定义标签
    tera.register_function("upper_case", custom_function);
    tera.register_filter("html_filter", html_filter);
    tera.register_function("format_time", time_function);
    tera.register_function("format_json", json_function);
    let _ = tera.full_reload();
    tera
}

pub fn json_to_hashmap(json: &Value) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    log::info!("======json1========：{:?}", &json.clone());
    match json {
        Value::Object(obj) => {
            log::info!("======obj========：{:?}", &obj.clone());
            for (key, value) in obj.iter() {
                if let Some(s) = value.as_str() {
                    log::info!("======k1========：{:?}", &key.clone());
                    hashmap.insert(key.clone(), s.to_string());
                } else {
                    let sub_hashmap = json_to_hashmap(value);
                    log::info!("======k2========：{:?}", &key.clone());
                    hashmap.insert(key.clone(), serde_json::to_string(&sub_hashmap).unwrap_or_default());
                }
            }
        }
        _ => {}
    }
    hashmap
}


pub fn json_function(args: &HashMap<String, Value>) -> Result<Value> {
    // 从args中获取模板传递的参数
    if let Some(value) = args.get("json") {

        let json_value: Value = serde_json::from_str(&value.to_string()).unwrap_or_default();
        let json_string = json_value.as_str().unwrap_or_default().to_string();
        //log::info!("json_string======================：{:?}", &json_string.clone());
        let json: Value = serde_json::from_str(&json_string)
            .map_err(|err| tera::Error::msg(format!("Failed to parse JSON: {}", err)))?;
        let hashmap =json_to_hashmap(&json);
        log::info!("json参数为：{:?}", &hashmap.clone());
        // 将结果包装为Tera的Value类型并返回
        return Ok(to_value(&hashmap).unwrap_or_default())
    }
    // 如果没有找到参数，则返回错误信息
    Err(Error::from("Missing parameter".to_string()))
}


pub fn custom_function(args: &HashMap<String, Value>) -> Result<Value> {
    // 从args中获取模板传递的参数
    if let Some(value) = args.get("param_name") {
        // 执行自定义逻辑
        let result = value.as_str().map(|s| s.to_uppercase());
        // 将结果包装为Tera的Value类型并返回
        return Ok(Value::String(result.unwrap_or_else(|| "".to_string())));
    }

    // 如果没有找到参数，则返回错误信息
    Err(Error::from("Missing parameter".to_string()))
}

pub fn html_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let re = match regex::Regex::new(r"<[^>]+>|&nbsp;") {
        Ok(re) => re,
        Err(_) => {
            log::error!("过滤html错误");
            return Err(Error::from("格式html错误".to_string()));
        }
    };
    let s = try_get_value!("html_filter", "value", String, value);
    Ok(to_value(re.replace_all(&s, "")).unwrap_or_default())
}

pub fn time_function(args: &HashMap<String, Value>) -> Result<Value> {
    // 从args中获取模板传递的参数
    if let Some(value) = args.get("time") {
        // 执行自定义逻辑
        let result = value.as_str().map(|s| s.to_uppercase());
        // 将结果包装为Tera的Value类型并返回
        return Ok(Value::String(compare_with_current_time(&result.unwrap_or_default())));
    }

    // 如果没有找到参数，则返回错误信息
    Err(Error::from("格式时间错误".to_string()))
}