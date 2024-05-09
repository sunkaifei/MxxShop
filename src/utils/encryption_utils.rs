//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use uuid::Uuid;

// md5
pub fn md5(data: &str) -> String {
    let mut h = Md5::new();
    h.input_str(data);
    h.result_str()
}

// sha1
pub fn sha1(data: &str) -> String {
    let mut h = Sha1::new();
    h.input_str(data);
    h.result_str()
}

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}









