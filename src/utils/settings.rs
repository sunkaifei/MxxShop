//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::{fs::File, io::Read};

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub server_url: String,
    pub jwt_secret: String,
    pub tpl_default: String,
    pub cache_type: String,
    pub debug: bool,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub mysql_url: String,
    pub redis_url: String,
    pub db_pool_len: usize,
    pub db_pool_timeout: usize,
}

#[derive(Debug, Deserialize)]
pub struct Attach {
    pub attach_path: String,
    pub upload_path: String,
    pub upload_url: String,
}

///时区
#[derive(Debug, Deserialize)]
pub struct Time {
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub server: Server,
    pub attach: Attach,
    pub time: Time,
}


impl Default for Settings {
    fn default() -> Self {
        let file_path = "./config/Config.toml";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e)
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(_) => (),
            Err(e) => panic!("Error Reading file: {}", e)
        };
        toml::from_str(&str_val).expect("Parsing the configuration file failed")
    }
}

impl Settings {
    pub fn get<'a>() -> &'a Self {
        // 给静态变量延迟赋值的宏
        lazy_static! {
            static ref CACHE: Settings = Settings::default();
        }
        &CACHE
    }
}