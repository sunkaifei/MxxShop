//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


use actix_cors::Cors;
use actix_http::header;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::{App, HttpServer};
use crate::core::errors::error::{Error, Result};
use actix_web::cookie::{Key};
use minijinja::value::{Rest, Value};
use minijinja::{context, path_loader, Environment, ErrorKind};
use minijinja_autoreload::AutoReloader;
use once_cell::sync::OnceCell;

#[allow(unused_imports)]
#[macro_use] 
extern crate rust_i18n;

use crate::core::service::{CONTEXT};
use crate::core::web::tags::i18n_tags::lang_function;
use crate::core::web::tags::url_tage::url_for;
use crate::routes::{open_routes, system_routes, user_routes};
use crate::utils::settings::Settings;

pub mod core;
pub mod utils;
pub mod modules;
pub mod routes;
pub mod plugins;

rust_i18n::i18n!("locales");

pub static TEMPLATE_WATCHER: OnceCell<AutoReloader> = OnceCell::new();
pub fn get_template(name: &str, ctx: Value) -> Result<String> {
    // let reloader = TEMPLATE_WATCHER.get().unwrap();
    // reloader.notifier().request_reload();
    // let env = reloader.acquire_env().unwrap();
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env.add_function("url_for", url_for);
    env.add_function("lang", lang_function);
    let tpl = env.get_template(name).unwrap();
    Ok(tpl.render(ctx).unwrap_or_default())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let setting = Settings::get();
    log4rs::init_file("./config/log4rs.yaml", Default::default()).unwrap_or_default();

    CONTEXT.init_database().await;
    log::info!("starting HTTP server at https://{:?}",&setting.server.server_url.as_str());
    // 创建一个 Redis 客户端
    //let client = Client::open("redis://127.0.0.1/").unwrap();
    //let connection = client.get_connection().unwrap();

    let signing_key = Key::generate();

    // let mut env = Environment::new();
    // env.set_loader(path_loader("templates"));
    // env.add_function("lang", url_for);


    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION,
                                  header::ACCEPT, header::ACCESS_CONTROL_ALLOW_ORIGIN])
            .allowed_header(header::CONTENT_TYPE)
            .allow_any_origin()
            .supports_credentials()
            .max_age(3600);

        App::new().wrap(cors)
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::new(CookieSessionStore::default(), signing_key.clone())
            )
            .configure(open_routes::configure_routes)
            .configure(user_routes::configure_routes)
            .configure(system_routes::configure_routes)
    })
        .bind(&setting.server.server_url)?
        .run()
        .await

}



