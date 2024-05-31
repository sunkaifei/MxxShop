//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

use crate::modules::system::entity::admin_entity::SystemAdmin;
use crate::modules::system::entity::menus_model::Router;
use crate::modules::system::entity::post_entity::SystemPost;
use crate::modules::system::entity::role_entity::SystemRole;
use crate::utils::string_utils::{serialize_option_u64_to_string,deserialize_string_to_u64};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSaveRequest {
    ///用户账号
    pub user_name: Option<String>,
    ///用户昵称
    pub nick_name: Option<String>,
    ///用户类型：0普通用户，1超级管理员
    pub user_type: Option<i8>,
    ///用户邮箱
    pub email: Option<String>,
    ///手机号码
    pub mobile: Option<String>,
    ///用户性别（0男 1女 2未知）
    pub sex: Option<i8>,
    ///头像地址
    pub avatar: Option<String>,
    ///密码
    pub password: Option<String>,
    ///帐号状态（0正常 1停用）
    pub status: Option<i8>,
    ///备注
    pub remark: Option<String>,
    ///用户排序
    pub sort: Option<i32>,
}

impl From<AdminSaveRequest> for SystemAdmin {
    fn from(req: AdminSaveRequest) -> Self {
        Self {
            id: None,
            user_name: req.user_name,
            nick_name: req.nick_name,
            user_type: req.user_type,
            email: req.email,
            mobile: req.mobile,
            sex: req.sex,
            avatar: req.avatar,
            password: req.password,
            status: req.status,
            del_flag: Option::from(0),
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: Option::from(DateTime::now()),
            update_by: None,
            update_time: Option::from(DateTime::now()),
            remark: req.remark,
            sort: req.sort,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
    pub user_type: Option<i8>,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    ///用户性别（0男 1女 2未知）
    pub sex: Option<i8>,
    pub login_ip: Option<String>,
    pub login_date: Option<DateTime>,
    pub sort: Option<i32>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

impl From<UserUpdateRequest> for SystemAdmin {
    fn from(req: UserUpdateRequest) -> Self {
        Self {
            id: req.id,
            user_name: req.user_name,
            user_type: req.user_type,
            email: req.email,
            mobile: req.mobile,
            sex: req.sex,
            avatar: req.avatar,
            password: None,
            status: req.status,
            del_flag: None,
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: Option::from(DateTime::now()),
            remark: req.remark,
            nick_name: req.nick_name,
            sort: req.sort,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserLoginResponse {
    pub token: String,
    #[serde(rename(serialize = "userInfo"))]
    pub user_info: SystemAdmin,
    #[serde(rename(serialize = "menuList"))]
    pub menu_list: Vec<Router>,
    pub permissions: Vec<String>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    //验证码
    pub verify_code: Option<String>,
    //验证码凭证，用于验证码校验
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryUserRoleReq {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserRoleData {
    pub sys_role_list: Vec<UserRoleList>,
    pub user_role_ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleList {
    pub id: u64,
    pub status: i32,
    pub sort: i32,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

/// 角色和岗位列表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleAndPostVO {
    pub role_list: Vec<SystemRole>,
    pub post_list: Vec<SystemPost>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuUserList {
    pub id: u64,
    pub parent_id: i32,
    pub name: String,
    pub path: String,
    pub api_url: String,
    pub menu_type: i32,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListRequest {
    pub page_num: u64,
    pub page_size: u64,
    pub admin_id: Option<u64>,
    pub user_name: Option<String>,
    pub mobile: Option<String>,
    pub depts_id: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListDTO {
    pub page_num: u64,
    pub page_size: u64,
    pub admin_id: Option<u64>,
    pub user_name: Option<String>,
    pub mobile: Option<String>,
    pub depts_id: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

impl From<UserListRequest>  for UserListDTO {
    fn from(req: UserListRequest) -> Self {
        Self {
            page_num: req.page_num,
            page_size: req.page_size,
            admin_id: req.admin_id,
            user_name: req.user_name,
            mobile: req.mobile,
            depts_id: req.depts_id,
            begin_time: req.begin_time,
            end_time: req.end_time,
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminListVO {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id:  Option<u64>,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
    ///用户昵称
    pub nick_name: Option<String>,
    ///所有拥有的权限组名称
    pub role_name: Option<Vec<Option<String>>>,
    ///所有的所在部门名称
    pub depts_name: Option<Vec<Option<String>>>,
    pub remark: Option<String>,
    pub sort:  Option<i32>,
    pub status:  Option<i8>,
    pub create_time: String,
    //pub update_time: String,
}



///用户更新密码结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdminPasswordRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: Option<u64>,
    pub password: Option<String>,
    pub re_password: Option<String>,
}

impl From<UpdateAdminPasswordRequest> for SystemAdmin {
    fn from(req: UpdateAdminPasswordRequest) -> Self {
        Self{
            id: req.user_id,
            user_name: None,
            nick_name: None,
            user_type: None,
            email: None,
            mobile: None,
            sex: None,
            avatar: None,
            password: req.password,
            status: None,
            del_flag: None,
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: None,
            sort: None,
        }
    }

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminVO {
    ///用户ID
    pub id: Option<u64>,
    ///用户账号
    pub user_name: Option<String>,
    ///用户昵称
    pub nick_name: Option<String>,
    ///用户类型：0普通用户，1超级管理员
    pub user_type: Option<i8>,
    ///用户邮箱
    pub email: Option<String>,
    ///手机号码
    pub mobile: Option<String>,
    ///用户性别（0男 1女 2未知）
    pub sex: Option<i8>,
    ///头像地址
    pub avatar: Option<String>,
    ///密码
    pub password: Option<String>,
    ///帐号状态（0正常 1停用）
    pub status: Option<i8>,
    ///删除标志（0代表存在 2代表删除）
    pub del_flag: Option<i8>,
    ///最后登陆IP
    pub login_ip: Option<String>,
    ///最后登陆时间
    pub login_date: Option<DateTime>,
    ///创建者
    pub create_by: Option<String>,
    ///创建时间
    pub create_time: Option<DateTime>,
    ///更新者
    pub update_by: Option<String>,
    ///更新时间
    pub update_time: Option<DateTime>,
    ///备注
    pub remark: Option<String>,
    ///用户排序
    pub sort: Option<i32>,
}

impl From<SystemAdmin> for SystemAdminVO {
    fn from(arg: SystemAdmin) -> Self {
        Self {
            id: arg.id,
            user_name: arg.user_name,
            nick_name: arg.nick_name,
            user_type: arg.user_type,
            email: arg.email,
            mobile: arg.mobile,
            sex: arg.sex,
            avatar: arg.avatar,
            password: arg.password,
            status: arg.status,
            del_flag: Option::from(0),
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: None,
            sort: Option::from(0),
        }
    }
}