//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::user::entity::user_entity::User;

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct UserSaveRequest {
    ///用户名
    //#[validate(length(min = 1), custom = "validate_unique_username")]
    pub user_name: Option<String>,
    ///用户手机
    //#[validate(phone)]
    pub mobile: Option<String>,
    ///用户密码
    #[validate(length(min = 6, max = 32))]
    pub password: Option<String>,
}

impl From<UserSaveRequest> for User{
    fn from(req: UserSaveRequest) -> Self {
        Self {
            id: None,
            short_url: None,
            user_name: req.user_name,
            email: None,
            mobile: req.mobile,
            password: None,
            salt: None,
            nick_name: None,
            avatar_file: None,
            sex: None,
            birthday: None,
            province: None,
            city: None,
            job_id: None,
            reg_time: Option::from(DateTime::now()),
            reg_ip: None,
            last_login: None,
            last_ip: None,
            online_time: None,
            last_active: None,
            notification_unread: None,
            inbox_unread: None,
            inbox_recv: None,
            now_money: None,
            integral: None,
            status: None,
            valid_email: None,
            is_del: None,
            reason: None,
            valid_mobile: None,
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct UserLoginRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponse {
    ///用户的 UID
    pub id: Option<u64>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginSession {
    ///用户的 UID
    pub id: Option<u64>,
    pub username: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo{
    ///用户的 UID
    pub id: Option<u64>,
    ///短网址
    pub short_url: Option<String>,
    ///用户名
    pub user_name: Option<String>,
    ///EMAIL
    pub email: Option<String>,
    ///用户手机
    pub mobile: Option<String>,
    ///昵称
    pub nick_name: Option<String>,
    ///头像文件
    pub avatar_file: Option<String>,
    ///性别
    pub sex: Option<i8>,
    ///生日
    pub birthday: Option<String>,
    ///省
    pub province: Option<u64>,
    ///市
    pub city: Option<u64>,
    ///未读系统通知
    pub notification_unread: Option<i8>,
    ///未读短信息
    pub inbox_unread: Option<i8>,
    ///0-所有人可以发给我,1-我关注的人
    pub inbox_recv: Option<i8>,
    ///手机认证
    pub valid_mobile: Option<String>,
}

impl From<User> for UserInfo {
    fn from(s: User) -> Self {
        Self {
            id: s.id,
            short_url: s.short_url,
            user_name: s.user_name,
            email: s.email,
            mobile: s.mobile,
            nick_name: s.nick_name,
            avatar_file: s.avatar_file,
            sex: s.sex,
            birthday: s.birthday,
            province: s.province,
            city: s.city,
            notification_unread: s.notification_unread,
            inbox_unread: s.inbox_unread,
            inbox_recv: s.inbox_recv,
            valid_mobile: s.valid_mobile,
        }
    }
}


