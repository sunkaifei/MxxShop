//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use crate::modules::system::entity::system_log_entity::SystemLog;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemLogSaveRequest {
    /// 模块标题
    pub title: Option<String>,
    /// 业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i32>,
    /// 方法名称
    pub method: Option<String>,
    /// 请求方式
    pub request_method: Option<String>,
    /// 操作类别（0其它 1后台用户 2手机端用户）
    pub operator_type: Option<i32>,
}


impl From<SystemLogSaveRequest> for SystemLog {
    fn from(item: SystemLogSaveRequest) -> Self {
        Self {
            id: None,
            title: item.title,
            business_type: item.business_type,
            method: item.method,
            request_method: item.request_method,
            operator_type: item.operator_type,
            oper_name: None,
            dept_name: None,
            oper_url: None,
            oper_ip: None,
            oper_location: None,
            oper_param: None,
            json_result: None,
            status: None,
            error_msg: None,
            create_time: Option::from(DateTime::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogPageRequest {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub title: Option<String>,
    pub business_type: Option<i32>,
    pub operator_type: Option<i32>,
    pub status: Option<i32>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogPageBO {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub title: Option<String>,
    pub business_type: Option<i32>,
    pub operator_type: Option<i32>,
    pub status: Option<i32>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

impl From<LogPageRequest> for LogPageBO {
    fn from(item: LogPageRequest) -> Self {
        Self {
            page_num: item.page_num,
            page_size: item.page_size,
            title: item.title,
            business_type: item.business_type,
            operator_type: item.operator_type,
            status: item.status,
            begin_time: item.begin_time,
            end_time: item.end_time,
        }
    }
}
