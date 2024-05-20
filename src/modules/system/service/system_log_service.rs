//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use core::option::Option;
use actix_web::HttpRequest;
use rbatis::rbdc::DateTime;
use crate::core::errors::error::Result;
use crate::modules::system::entity::system_log_entity::SystemLog;
use crate::pool;
use crate::utils::snowflake_id::{generate_snowflake_id};

/// #添加系统日志
/// * `request`  -  请求对象
/// * `title` - 日志标题
/// * `business_type` -业务类型（0查看 1新增 2修改 3删除）
/// * `method` - 方法名称
/// * `request_method` - 请求方式
/// * `operator_type` - 操作类别（0其它 1后台用户 2手机端用户）
///
pub async fn save_system_log(request: &HttpRequest,
                             title: Option<String>,
                             business_type: Option<i32>,
                             method: Option<String>,
                             request_method: Option<String>,
                             operator_type: Option<i32>,
) -> Result<u64> {

    //获取请求的接口
    let uri = request.uri();
    let url = format!("{}{}", uri.path(), uri.query().map(|q| format!("?{}", q)).unwrap_or_default());

    let system_log = SystemLog {
        id: Some(generate_snowflake_id()),
        title,
        business_type,
        method,
        request_method,
        operator_type,
        oper_name: None,
        dept_name: None,
        oper_url: Option::from(url),
        oper_ip: Option::from(request.connection_info().realip_remote_addr().unwrap_or_default().to_string()),
        oper_location: None,
        oper_param: None,
        json_result: None,
        status: Some(0),
        error_msg: None,
        create_time: Option::from(DateTime::now()),
    };
    let result = SystemLog::insert(pool!(), &system_log).await;
    return Ok(result?.rows_affected);
}


///批量删除系统日志
pub async fn bath_delete_type(ids: Vec<Option<String>>) -> Result<u64> {
    let result = SystemLog::delete_in_column(pool!(), "id", &ids).await;
    return Ok(result?.rows_affected);
}