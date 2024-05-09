//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::future::{ready, Ready};
use std::rc::Rc;

use actix_http::HttpMessage;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use actix_web::error;
use futures_util::future::LocalBoxFuture;
use serde_json::json;

use crate::core::permission::jwt_util::JWTToken;
use crate::utils::settings::Settings;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service)
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let path = req.path().to_string();
        //log::info!("==========permission======== params: {:?}", &permission);
        let token = req
            .headers()
            .get("Authorization")
            .map(|v| v.to_str().unwrap_or_default().to_string())
            .unwrap_or_default()
            .split("Bearer ")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap_or_default()
            .to_string();

        return Box::pin(async move {
            log::info!("Hi from start. You requested path: {}", path);

            if path.contains("login") {
                let fut = svc.call(req);
                let res = fut.await?;
                return Ok(res);
            }
            if path.contains("logout") {
                let fut = svc.call(req);
                let res = fut.await?;
                return Ok(res);
            }
            if token.len() <= 0 {
                let john = json!({
                        "message": "token不能为空",
                        "code": 401,
                        "data": "None"
                    });
                return Err(error::ErrorUnauthorized(john.to_string()));
            }

            if let Some(permission) = req.extensions().get::<String>() {
                println!("拦截器权限拦截测试---有权限: {}", permission);
                // 这里可以根据permission的值进行逻辑处理
            } else {
                println!("拦截器权限拦截测试---无权限");
            }


            //log::info!("++++++++++++++++++++++++++: {}", &token);
            let setting = Settings::get();
            let jwt_token_e = JWTToken::verify(&setting.server.jwt_secret, &token);

            match jwt_token_e {
                Ok(data) => data,
                Err(err) => {
                    //log::info!("+++++++++++++1+++++++++++++");
                    let john = json!({
                        "message": "无权限访问",
                        "code": 401,
                        "data": err.to_string()
                    });
                    return Err(error::ErrorUnauthorized(john.to_string()));
                }
            };


            let res = svc.call(req).await?;

            //log::info!("+++++++++++++2+++++++++++++");
            Ok(res)
        });


        /*let fut = svc.call(req);
        let res = fut.await?;
        Ok(res)*/
        /* let jwt_token_e = JWTToken::verify("123", &token);
         let jwt_token = match jwt_token_e {
             Ok(data) => { data }
             Err(err) => {
                 let er = match err {
                     WhoUnfollowedError::JwtTokenError(s) => { s }
                     _ => "no math error".to_string()
                 };
                 let john = json!({
                     "message": er,
                     "code": 401,
                     "path": path
                 });
                 log::error!("Hi from start. You requested path: {}, token: {}", path, token);
                 return Err(error::ErrorUnauthorized(john.to_string()));
             }
         };*/
        /*
                    let mut flag: bool = false;
                    for token_permission in &jwt_token.permissions {
                        if token_permission == &path {
                            flag = true;
                            break;
                        }
                    }

                    if flag {
                        let fut = svc.call(req);
                        let res = fut.await?;
                        Ok(res)
                    } else {
                        log::error!("Hi from start. You requested path: {:?}", &jwt_token.permissions);
                        let john = json!({
                            "message": "无权限访问",
                            "code": 401,
                            "path": path
                        });
                        Err(error::ErrorUnauthorized(john.to_string()))
                    }*/

        /* let fut = svc.call(req);
         let res = fut.await?;
         Ok(res)*/
    }
}