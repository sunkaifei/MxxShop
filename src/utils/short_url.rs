//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::collections::HashSet;

use base64::Engine;
use base64::engine::general_purpose;
use uuid::Uuid;

pub fn generate_random_code(length: usize) -> Option<String> {
    if length < 5 {
        return None;
    }

    let mut unique_codes = HashSet::new();

    loop {
        // Generate a random UUID and convert it to string format
        let uuid_str = Uuid::new_v4().to_string();

        // Convert UUID to base64 encoded string
        let code = general_purpose::URL_SAFE_NO_PAD.encode(uuid_str);
        // Extract substring of the desired length from the base64 encoded string
        let code = code.chars().take(length).collect::<String>();

        // Check if this code has already been generated
        if unique_codes.contains(&code) {
            continue;
        }

        // Add the new code to the set of unique codes
        unique_codes.insert(code.clone());

        // Return the new code
        return Some(code);
    }
}
