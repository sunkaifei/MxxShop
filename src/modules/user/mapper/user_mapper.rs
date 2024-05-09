//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{htmlsql, impl_select, py_sql};
use rbatis::RBatis;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;

use crate::modules::user::entity::user_entity::User;
use crate::modules::user::entity::user_model::UserSaveDTO;

rbatis::crud!(User{}, "fly_users");

impl_select!(User{select_by_username(username: &Option<String>) -> Option => "`where user_name = #{username} limit 1`"},"fly_users");


htmlsql!(add_user(rb: &RBatis, user: &UserSaveDTO) -> Result<ExecResult, Error> =>
r#"<mapper>
    <insert id="add_user">
        `insert into fly_users(`
        `<if test="user.id != null && user.id != 0">id,</if>
		<if test="user.user_name != null && user.user_name != ''">user_name,</if>
		<if test="user.email != null && user.email != ''">email,</if>
		<if test="user.mobile != null && user.mobile != ''">mobile,</if>
		<if test="user.password != null && user.password != ''">password,</if>
		<if test="user.salt != null && user.salt != ''">salt,</if>
        <if test="user.reg_time != null && user.reg_time != ''">reg_time,</if>
		<if test="user.reg_ip != null && user.reg_ip != ''">reg_ip,</if>
		<if test="user.last_login != null && user.last_login != ''">last_login,</if>
		<if test="user.last_ip != null && user.last_ip != ''">last_ip,</if>
		<if test="user.skin != null && user.skin != ''">skin,</if>`
    `)values(`
       `<if test="user.id != null && user.id != ''">#{user.id},</if>
		<if test="user.user_name != null && user_name != ''">#{user.user_name},</if>
		<if test="user.email != null && user.email != ''">#{user.email},</if>
		<if test="user.mobile != null && user.mobile != ''">#{user.mobile},</if>
		<if test="user.password != null && user.password != ''">#{user.password},</if>
		<if test="user.salt != null && user.salt != ''">#{user.salt},</if>
		<if test="user.reg_time != null && user.reg_time != ''">#{user.reg_time},</if>
		<if test="user.reg_ip != null && user.reg_ip != ''">#{user.reg_ip},</if>
		<if test="user.last_login != null && user.last_login != ''">#{user.last_login},</if>
		<if test="user.last_ip != null && user.last_ip != ''">#{user.last_ip},</if>
		<if test="user.skin != null && user.skin != ''">#{user.skin},</if>`
	`)`
    </insert>
</mapper>"#);

//in用户id查出用户信息
#[py_sql("
    `select * from fly_users`
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


