use actix_web::{HttpRequest};
use crate::core::permission::jwt_util::JWTToken;
use crate::utils::error::WhoUnfollowedError;
use crate::utils::settings::Settings;

pub fn get_user(req: HttpRequest) -> Result<JWTToken, WhoUnfollowedError> {
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
    let setting = Settings::get();
    let jwt_token = JWTToken::verify(&setting.server.jwt_secret, &token)?;
    Ok(jwt_token)
}

