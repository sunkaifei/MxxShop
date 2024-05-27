//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use rbatis::Error;
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbs::Value;

#[derive(Debug)]
pub struct AuthorityIdPlugin{
    pub args: u32,
}

impl AuthorityIdPlugin {
    pub fn new() -> Self {
        AuthorityIdPlugin { args: 0 }
    }
}

#[rbatis::async_trait]
impl Intercept for AuthorityIdPlugin {
    async fn before(
        &self,
        _task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        //log::info!("=============args====================: {:?}", args);
        if sql.contains("delete from ") {
            // 拦截删除语句并转换为更新删除标志的语句
            // ...
            log::info!("拦截删除语句: {}", sql);
        } else if sql.contains("from mxx_system_menu"){
            // 添加过滤条件
            // ...
            // 修改 SQL 语句，使其只返回特定的列
            //sql.push_str(" limit 10");
            log::info!("拦截查询语句: {}", sql);
        }
        Ok(Some(true))
    }
}
