//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{impl_select, py_sql};
use rbatis::RBatis;

use crate::modules::user::entity::user_entity::User;

rbatis::crud!(User{}, "mxx_user");

impl_select!(User{select_by_username(username: &Option<String>) -> Option => "`where user_name = #{username} limit 1`"},"mxx_user");


//in用户id查出用户信息
#[py_sql("
    `select * from mxx_user`
    ` where id in (`
     trim ',': for item in user_id:
       #{item},
     )
    ` and forbidden = 0`")]
pub async fn get_user_by_in_id(
    rb: &RBatis,
    user_id: Vec<u64>,
) -> rbatis::Result<Vec<User>> {
    impled!()
}


