//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::fmt::Debug;
use std::format;
use std::time::Duration;

use futures_util::future::BoxFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::errors::error::{Result};
use crate::modules::system::service::mem_service::MemService;
use crate::modules::system::service::redis_service::RedisService;

use crate::utils::settings::Settings;

pub trait ICacheService: Sync + Send+ Debug {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>>;

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>>;

    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<Result<String>>;

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>>;
    fn del(&self, k: &str) -> BoxFuture<Result<i64>>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new() -> Result<Self> {
        let setting = Settings::get();
        match setting.server.cache_type.as_str() {
            "mem" => {
                println!("[MxxShop_admin] cache_type: mem");
                Ok(Self {
                    inner: Box::new(MemService::default()),
                })
            }
            "redis" => {
                println!("[MxxShop_admin] cache_type: redis");
                Ok(Self {
                    inner: Box::new(RedisService::new(&setting.database.redis_url)),
                })
            }
            e => {
                panic!(
                    "[MxxShop_admin] unknown of cache_type: \"{}\",current support 'mem' or 'redis'",
                    e
                );
            }
        }
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.inner.set_string(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        self.inner.get_string(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(crate::core::errors::error::Error::from(format!(
                "MemCacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string(k, data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(crate::core::errors::error::Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        self.inner.ttl(k).await
    }


    pub async fn del(&self, k: &str) -> Result<i64> {
        self.inner.del(k).await
    }
}
