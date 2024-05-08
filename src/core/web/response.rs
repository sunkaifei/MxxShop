use std::fmt::Debug;
use crate::core::errors::error::Error;
use serde::{Deserialize, Serialize};

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct ResVO<T>
    where T: Serialize + Debug
{
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize + Debug> ResVO<T> {
    // 处理统一返回
    pub fn handle_result(result: Result<u64, Error>) -> ResVO<T> {
        if result.is_ok(){
            match result {
                Ok(data) => {
                    if data > 0 {
                        Self::ok_msg("更新成功".to_string()) 
                    }else{
                        ResVO::error_msg("更新失败".to_string())
                    }
                },
                Err(e) => Self::error_msg(e.to_string()),
            }
        }else{
            ResVO::error_msg(result.err().unwrap().to_string())
        }
    }
    
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        ResVO {
            code,
            message,
            data,
        }
    }

    pub fn ok() -> Self {
        Self::new(200, "操作成功".to_string(), None)
    }
    
    pub fn ok_msg(message: String) -> Self {
        Self::new(200, message, None)
    }

    pub fn ok_with_data(data: T) -> Self {
        Self::new(200, "操作成功".to_string(), Some(data))
    }

    pub fn error(message: String, code: i32) -> Self {
        Self::new(code, message, None)
    }
    
    pub fn error_msg(message: String) -> Self {
        Self::new(400, message, None)
    }
    
    pub fn error_with_data(message: String, code: i32, data: T) -> Self {
        Self::new(code, message, Some(data))
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultPage<T> {
    pub current_page: u64,
    pub list: T,
    pub total: u64,
}

// 统一返回分页
#[derive(Serialize, Debug, Clone)]
pub struct ResponsePage<T>
    where
        T: Serialize + Debug,
{
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

pub fn ok_result_page<T: Serialize + Debug>(data: T) -> ResponsePage<T> {
    ResponsePage {
        message: "操作成功".to_string(),
        code: 200,
        data: Some(data),
    }
}

pub fn err_result_page(message: String) -> ResponsePage<String> {
    ResponsePage {
        message: message.to_string(),
        code: 400,
        data: Some("None".to_string()),
    }
}

