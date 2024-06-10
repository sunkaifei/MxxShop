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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct User {
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
    ///用户密码
    pub password: Option<String>,
    ///用户附加混淆码
    pub salt: Option<String>,
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
    ///职业ID
    pub job_id: Option<u64>,
    ///注册时间
    pub reg_time: Option<DateTime>,
    ///注册IP
    pub reg_ip: Option<String>,
    ///最后登录时间
    pub last_login: Option<DateTime>,
    ///最后登录 IP
    pub last_ip: Option<String>,
    ///在线时间
    pub online_time: Option<DateTime>,
    ///最后活跃时间
    pub last_active: Option<DateTime>,
    ///未读系统通知
    pub notification_unread: Option<i8>,
    ///未读短信息
    pub inbox_unread: Option<i8>,
    ///0-所有人可以发给我,1-我关注的人
    pub inbox_recv: Option<i8>,
    ///用户余额
    pub now_money: Option<i32>,
    ///用户剩余积分
    pub integral: Option<i32>,
    ///是否禁止用户,0-正常,1-禁止
    pub status: Option<i8>,
    ///邮箱验证
    pub valid_email: Option<i8>,
    ///是否删除0正常1删除
    pub is_del: Option<i8>,
    ///删除/封禁原因
    pub reason: Option<String>,
    ///手机认证
    pub valid_mobile: Option<String>,
}
