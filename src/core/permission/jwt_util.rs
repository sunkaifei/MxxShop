//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, errors::ErrorKind, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::error::WhoUnfollowedError;
use crate::utils::error::WhoUnfollowedError::JwtTokenError;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub permissions: Vec<String>,
    aud: String,
    // (audience)：受众
    exp: usize,
    iat: usize,
    // (Issued At)：签发时间
    iss: String,
    // (issuer)：签发人
    nbf: usize,
    // (Not Before)：生效时间
    sub: String,
    // (subject)：主题
    jti: String, // (JWT ID)：编号
}

impl JWTToken {
    pub fn new(id: Option<u64>, username: Option<String>, permissions: Vec<String>) -> JWTToken {
        let now = SystemTime::now();
        //过期时间
        let m30 = Duration::from_secs(1800000);
        let now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");

        JWTToken {
            id,
            username,
            permissions,
            aud: String::from("rust_admin"), // (audience)：受众
            exp: (now + m30).as_secs() as usize,
            iat: now.as_secs() as usize,  // (Issued At)：签发时间
            iss: String::from("flycms"),     // (issuer)：签发人
            nbf: now.as_secs() as usize,  // (Not Before)：生效时间
            sub: String::from("rust_admin"), // (subject)：主题
            jti: String::from("ignore"),  // (JWT ID)：编号
        }
    }

    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, WhoUnfollowedError> {
        let mut validation = Validation::default();
        validation.validate_nbf = true;
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(JwtTokenError("create token error".to_string())),
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, WhoUnfollowedError> {
        let mut validation = Validation::default();
        validation.sub = Some("rust_admin".to_string());
        validation.set_audience(&["rust_admin"]);
        validation.validate_nbf = true;
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),

            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(JwtTokenError("InvalidToken".to_string())), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(JwtTokenError("InvalidIssuer".to_string())), // Example on how to handle a specific error
                ErrorKind::ExpiredSignature => return Err(JwtTokenError("token 已经超时了".to_string())), // Example on how to handle a specific error
                // _ => return Err(Error::from("InvalidToken other errors")),
                _ => Err(JwtTokenError("create token error".to_string())),
            },
        };
    }

    pub fn refresh(&self, secret: &str, jwt_exp: usize) -> Result<String, WhoUnfollowedError> {
        let mut jwt = self.clone();
        jwt.exp = jwt.exp + jwt_exp;
        jwt.create_token(&secret)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::permission::jwt_util::JWTToken;

    #[test]
    fn test_jwt() {
        let jwt = JWTToken::new(Some(1), Some("koobe".to_string()), vec![]);
        let res = jwt.create_token("123");
        match res {
            Ok(token) => {
                println!("{:?}", token);

                let token_s = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwidXNlcm5hbWUiOiJrb29iZSIsInBlcm1pc3Npb25zIjpbXSwiYXVkIjoicnVzdF9hZG1pbiIsImV4cCI6MTY5OTc4NDk3MywiaWF0IjoxNjk3OTg0OTczLCJpc3MiOiJrb29iZSIsIm5iZiI6MTY5Nzk4NDk3Mywic3ViIjoicnVzdF9hZG1pbiIsImp0aSI6Imlnbm9yZSJ9.NJ6mIJtBAedyzOgqEnoLk8Cs2GqQ33G6w8V-aVpY-WQ";
                let token_tk = JWTToken::verify("123", &token_s);
                println!("{:?}", token_tk)
            }
            Err(_) => {}
        }
    }
}