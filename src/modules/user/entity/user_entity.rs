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
    pub password: String,
    ///用户附加混淆码
    pub salt: Option<String>,
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
    ///职业ID
    pub job_id: u64,
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
    pub notification_unread: i8,
    ///未读短信息
    pub inbox_unread: i8,
    ///0-所有人可以发给我,1-我关注的人
    pub inbox_recv: i8,
    ///粉丝数
    pub fans_count: i32,
    ///观众数
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
    ///邀请数量
    pub invitation_available: i32,
    ///用户组
    pub group_id: u64,
    ///威望对应组
    pub reputation_group: i32,
    ///是否禁止用户,0-正常,1-禁止
    pub forbidden: i8,
    ///邮箱验证
    pub valid_email: i8,
    ///首次登录标记
    pub is_first_login: i32,
    ///赞同数量
    pub agree_count: i32,
    ///感谢数量
    pub thanks_count: i32,
    ///个人主页查看数量
    pub views_count: i32,
    ///威望
    pub reputation: i32,
    ///威望更新
    pub reputation_update_time: Option<DateTime>,
    ///微博允许访问
    pub weibo_visit: i32,

    pub integral: i64,
    ///常用邮箱
    pub common_email: Option<String>,

    ///主题
    pub theme: Option<String>,
    ///专栏数量
    pub column_count: i32,
    ///皮肤
    pub skin: Option<String>,
    ///是否删除0正常1删除
    pub is_del: i8,
    ///删除/封禁原因
    pub reason: Option<String>,
    ///手机认证
    pub valid_mobile: Option<String>,
}
