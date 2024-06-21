//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use std::cell::RefCell;
use actix_web::HttpRequest;
use minijinja::{Error, ErrorKind, Value};
use minijinja::value::Rest;

thread_local! {
    pub static CURRENT_REQUEST: RefCell<Option<HttpRequest>> = RefCell::default()
}

fn with_bound_req<F, R>(req: &HttpRequest, f: F) -> R
    where
        F: FnOnce() -> R,
{
    CURRENT_REQUEST.with(|current_req| *current_req.borrow_mut() = Some(req.clone()));
    let rv = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    CURRENT_REQUEST.with(|current_req| current_req.borrow_mut().take());
    match rv {
        Ok(rv) => rv,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}
pub fn url_for(name: &str, args: Rest<String>) -> Result<Value, Error> {
    CURRENT_REQUEST.with(|current_req| {
        Ok(current_req
            .borrow()
            .as_ref()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    "url_for requires an http request",
                )
            })?
            .url_for(name, &args[..])
            .map_err(|err| {
                Error::new(ErrorKind::InvalidOperation, "failed to generate url").with_source(err)
            })?
            .to_string()
            .into())
    })
}