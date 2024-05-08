use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SystemAdmin {
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
