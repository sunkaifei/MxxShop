//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!
use std::sync::Arc;
use std::time::Duration;
use once_cell::sync::Lazy;
use rbatis::RBatis;
use crate::plugins::authority_intercept::AuthorityIdPlugin;
use crate::utils::settings::Settings;

pub mod templates_service;

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::core::service::CONTEXT.rb
    };
}
pub struct ServiceContext {
    pub rb: RBatis,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_database(&self) {
        let setting = Settings::get();
        self.rb.init(rbdc_mysql::driver::MysqlDriver {}, &setting.database.mysql_url, ).unwrap();
        self.rb.intercepts.push(Arc::new(AuthorityIdPlugin::new()));
        self.rb.get_pool().unwrap().set_max_open_conns(setting.database.db_pool_len as u64).await;
        self.rb.get_pool().unwrap().set_timeout(Some(Duration::from_secs(setting.database.db_pool_timeout as u64))).await;
        log::info!(
            "[mxxshop_admin] rbatis pool init success! pool state = {}",
            self.rb.get_pool().expect("pool not init!").state().await
        );

    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            rb: {
                let rb = RBatis::new();
                let setting = Settings::get();
                if rb.is_debug_mode() == false && setting.server.debug.eq(&true) {
                    panic!(r#"please edit config/Config.toml file   “debug: false” "#);
                }
                rb
            },
        }
    }
}