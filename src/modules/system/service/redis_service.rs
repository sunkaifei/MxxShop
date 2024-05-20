//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::time::Duration;

use futures_util::future::BoxFuture;
use log::error;
use redis::aio:: MultiplexedConnection;
use redis::RedisResult;

use crate::core::errors::error::{Error, Result};
use crate::modules::system::service::cache_service::ICacheService;

///Redis Cache service
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        println!("[MxxShop_admin] connect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        println!("[MxxShop_admin] connect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<MultiplexedConnection> {
        let conn = self.client.get_multiplexed_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(crate::core::errors::error::Error::from(err));
        }
        return Ok(conn.unwrap());
    }
}

impl ICacheService for RedisService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            return self.set_string_ex(&k, &v, None).await;
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let result: RedisResult<Option<String>> =
                redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
            return match result {
                Ok(v) => Ok(v.unwrap_or_default()),
                Err(e) => Err(Error::from(format!(
                    "RedisService get_string({}) fail:{}",
                    k,
                    e.to_string()
                ))),
            };
        })
    }

    ///set_string Automatically expire
    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return if ex.is_none() {
                match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            } else {
                match redis::cmd("SET")
                    .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                    .query_async(&mut conn)
                    .await
                {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            };
        })
    }

    ///set_string Automatically expire
    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService ttl fail:{}",
                    e.to_string()
                ))),
            };
        })
    }
    
    fn del(&self, k: &str) -> BoxFuture<Result<i64>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("DEL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService del fail:{}",
                    e.to_string()
                ))),
            };
        })
    }
}
