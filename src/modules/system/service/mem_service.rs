use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::Sub;
use std::time::{Duration, Instant};

use futures_util::future::BoxFuture;
use parking_lot::Mutex;

use crate::core::errors::error::{Error, Result};
use crate::modules::website::service::cache_service::ICacheService;

///Memory Cache Service
pub struct MemService {
    pub cache: Mutex<HashMap<String, (String, Option<(Instant, Duration)>), RandomState>>,
}

impl MemService {
    pub fn recycling(&self) {
        let mut map_lock_guard = self.cache.lock();
        let mut need_removed = vec![];
        for (k, v) in map_lock_guard.iter() {
            if let Some((i, d)) = v.1 {
                if i.elapsed() >= d {
                    //out of time
                    need_removed.push(k.to_string());
                }
            }
        }
        for x in need_removed {
            map_lock_guard.remove(&x);
        }
    }
}

impl Default for MemService {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

impl ICacheService for MemService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut guard = self.cache.lock();
        guard.insert(k.to_string(), (v.clone(), None));
        Box::pin(async move {
            return Ok(v.to_string());
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let guard = self.cache.lock();
        let mut v = String::new();
        if let Some(r) = guard.get(&k) {
            v = r.0.to_string();
        }
        Box::pin(async move { Ok(v) })
    }

    fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut locked = self.cache.lock();
        let mut e = Option::None;
        if let Some(ex) = t {
            e = Some((Instant::now(), ex));
        }
        let inserted = locked.insert(k, (v.clone(), e));
        Box::pin(async move {
            if inserted.is_some() {
                return Ok(v.to_string());
            }
            return Err(crate::core::errors::error::Error::E("[fly_admin][mem_service]insert fail!".to_string()));
        })
    }

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        self.recycling();
        let locked = self.cache.lock();
        let v = locked.get(k).cloned();
        drop(locked);
        let v = match v {
            None => -2,
            Some((_r, o)) => match o {
                None => -1,
                Some((i, d)) => {
                    let use_time = i.elapsed();
                    if d > use_time {
                        d.sub(use_time).as_secs() as i64
                    } else {
                        0
                    }
                }
            },
        };
        Box::pin(async move { Ok(v) })
    }

    fn del(&self, k: &str) -> BoxFuture<Result<i64>> {
        self.recycling();
        let k = k.to_string();
        let mut guard = self.cache.lock();

        if guard.remove(&k).is_some() {
            Box::pin(async { Ok(1) })
        } else {
            let error_msg = "[fly_admin][mem_service] delete failed!";
            Box::pin(async { Err(Error::from(error_msg.to_string())) })
        }
    }
}
