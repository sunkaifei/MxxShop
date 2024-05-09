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
use crate::modules::system::entity::menu_model::Router;
use crate::utils::string_utils::{serialize_option_u64_to_string};

#[derive(Debug, Deserialize)]
pub struct AdminSaveRequest {
    pub mobile: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    ///密码
    pub password: Option<String>,
    pub user_type: Option<i8>,
    pub email: Option<String>,
    pub sort: Option<i32>,
    pub remark: Option<String>,
    pub status: Option<i8>,
}

impl From<AdminSaveRequest> for SystemAdmin {
    fn from(req: AdminSaveRequest) -> Self {
        Self {
            id: None,
            dept_id: None,
            user_name: req.user_name,
            nick_name: req.nick_name,
            user_type: req.user_type,
            email: req.email,
            mobile: req.mobile,
            sex: Option::from(0),
            avatar: None,
            password: req.password,
            status: Option::from(0),
            del_flag: Option::from(0),
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

#[derive(Debug, Deserialize)]
pub struct UserUpdateRequest {
    pub id: Option<u64>,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
    pub user_type: Option<i8>,
    pub sort: Option<i32>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

impl From<UserUpdateRequest> for SystemAdmin {
    fn from(req: UserUpdateRequest) -> Self {
        Self {
            id: req.id,
            dept_id: None,
            user_name: req.user_name,
            user_type: req.user_type,
            email: None,
            mobile: req.mobile,
            sex: Option::from(0),
            avatar: None,
            password: None,
            status: Option::from(0),
            del_flag: Option::from(0),
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: None,
            remark: req.remark,
            nick_name: None,
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

#[derive(Debug, Deserialize)]
pub struct QueryUserMenuReq {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct QueryUserMenuResp {
    pub msg: String,
    pub code: i32,
    pub data: QueryUserMenuData,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserMenuData {
    pub sys_menu: Vec<MenuUserList>,
    pub permissions: Vec<String>,
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
pub struct UserListRequest {
    #[serde(rename = "pageNum")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub admin_id: Option<u64>,
    pub user_name: Option<String>,
    pub mobile: Option<String>,
    //#[serde(rename = "deptId")]
    pub dept_id: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct UserListResp {
    pub msg: String,
    pub code: i32,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<UserListData>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListData {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id:  Option<u64>,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
    ///用户昵称
    pub nick_name: Option<String>,
    pub remark: Option<String>,
    pub sort:  Option<i32>,
    pub status:  Option<i8>,
    pub create_time: String,
    //pub update_time: String,
}



///用户更新密码结构体
#[derive(Debug, Deserialize)]
pub struct UpdateUserPasswordRequest {
    pub id: Option<u64>,
    pub password: Option<String>,
    pub re_password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminVO {
    ///用户ID
    pub id: Option<u64>,
    ///部门ID
    pub dept_id: Option<u64>,
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
            dept_id: arg.dept_id,
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