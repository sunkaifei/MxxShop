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
    pub user_name: String,
    ///用户手机
    //#[validate(phone)]
    pub mobile: Option<String>,
    ///用户密码
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSaveDTO {
    ///用户的 UID
    pub id: u64,
    ///用户名
    pub user_name: String,
    ///EMAIL
    pub email: Option<String>,
    ///用户手机
    pub mobile: Option<String>,
    ///用户密码
    pub password: String,
    ///用户附加混淆码
    pub salt: Option<String>,
    ///注册时间
    pub reg_time: Option<DateTime>,
    ///注册IP
    pub reg_ip: Option<String>,
    ///最后登录时间
    pub last_login: Option<DateTime>,
    ///最后登录 IP
    pub last_ip: Option<String>,
    ///皮肤
    pub skin: Option<String>,
    ///是否删除0正常1删除
    pub is_del: i8,
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
    pub user_name: String,
    ///EMAIL
    pub email: Option<String>,
    ///用户手机
    pub mobile: Option<String>,
    ///昵称
    pub nick_name: Option<String>,
    ///头像文件
    pub avatar_file: Option<String>,
    ///性别
    pub sex: i8,
    ///生日
    pub birthday: Option<String>,
    ///省
    pub province: i64,
    ///市
    pub city: i64,
    ///未读系统通知
    pub notification_unread: i8,
    ///未读短信息
    pub inbox_unread: i8,
    ///0-所有人可以发给我,1-我关注的人
    pub inbox_recv: i8,
    ///粉丝数
    pub fans_count: i32,
    ///关注数
    pub friend_count: i32,
    ///邀请我回答数量
    pub invite_count: i32,
    ///文章数量
    pub article_count: i32,
    ///问题数量
    pub question_count: i32,
    ///回答数量
    pub answer_count: i32,
    ///关注话题数量
    pub topic_focus_count: i32,
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
            fans_count: s.fans_count,
            friend_count: s.friend_count,
            invite_count: s.invite_count,
            article_count: s.article_count,
            question_count: s.question_count,
            answer_count: s.answer_count,
            topic_focus_count: s.topic_focus_count,
            valid_mobile: s.valid_mobile,
        }
    }
}


