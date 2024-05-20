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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemLog {
    /// 日志主键
    pub id: Option<u64>,
    /// 模块标题
    pub title: Option<String>,
    /// 业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i32>,
    /// 方法名称
    pub method: Option<String>,
    /// 请求方式(POST, PUT, DELETE)
    pub request_method: Option<String>,
    /// 操作类别（0其它 1后台用户 2手机端用户）
    pub operator_type: Option<i32>,
    /// 操作人员
    pub oper_name: Option<String>,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 请求URL
    pub oper_url: Option<String>,
    /// 主机地址
    pub oper_ip: Option<String>,
    /// 操作地点
    pub oper_location: Option<String>,
    /// 请求参数
    pub oper_param: Option<String>,
    /// 返回参数
    pub json_result: Option<String>,
    /// 操作状态（0正常 1异常）
    pub status: Option<i32>,
    /// 错误消息
    pub error_msg: Option<String>,
    /// 操作时间
    pub create_time: Option<DateTime>,
}
