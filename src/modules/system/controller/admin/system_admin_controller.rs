//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

extern crate bcrypt;

use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, web};
use actix_web_grants::protect;
use bcrypt::{DEFAULT_COST, hash, verify};
use rbatis::rbdc::DateTime;

use crate::core::permission::jwt_util::JWTToken;
use crate::core::web::entity::common::{BathIdRequest, InfoId};
use crate::core::web::response::{ok_result_page, ResultPage, ResVO};
use crate::modules::system::entity::admin_model::{AdminSaveRequest, SystemAdminVO, UpdateAdminPasswordRequest, AdminListVO, UserListRequest, UserLoginRequest, UserLoginResponse, AdminUpdateRequest, RoleAndPostVO, UpdateAdminStatusRequest, RoleNameDTO, DeptsNameDTO, UpdateAdminRoleRequest, UpdateLoginRequest};
use crate::modules::system::entity::menus_model::{Router};
use crate::modules::system::service::{admin_service, depts_service, menus_service, post_service, role_service, system_log_service};
use crate::core::errors::error::WhoUnfollowedError;
use crate::core::service::CONTEXT;
use crate::core::web::base_controller::get_user;
use crate::modules::system::entity::admin_entity::SystemAdmin;
use crate::utils::settings::Settings;

// 添加用户信息
#[post("/system/admin/save_admin")]
pub async fn save_admin(item: web::Json<AdminSaveRequest>) -> HttpResponse {
    if item.user_name.as_ref().map_or(true, |username| username.trim().is_empty()) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("用户名称不能为空".to_string()));
    }
    if item.password.as_ref().map_or(true, |password| password.trim().is_empty()) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("密码不能为空".to_string()));
    }
    if admin_service::find_by_name_unique(&item.user_name, &None).await.unwrap_or_default(){
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("用户名已存在".to_string()));
    }
    if admin_service::find_by_mobile_unique(&item.mobile, &None).await.unwrap_or_default(){
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("手机号已存在".to_string()));
    }
    if item.email.is_some() {
        if admin_service::find_by_email_unique(&item.email, &None).await.unwrap_or_default(){
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("邮箱已存在".to_string()));
        }
    }
    if admin_service::find_by_nick_name_unique(&item.nick_name, &None).await.unwrap_or_default(){
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("昵称已存在".to_string()));
    }
    let result = admin_service::save_admin(item.0).await;
    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result));
}

/// 后台用户登录
#[post("/system/login")]
pub async fn login(request: HttpRequest, item: web::Json<UserLoginRequest>) -> HttpResponse {
    if item.username.as_ref().map_or(true, |username| username.trim().is_empty()) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("角色名称不能为空".to_string()));
    }
    if item.password.as_ref().map_or(true, |password| password.trim().is_empty()) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("角色名称不能为空".to_string()));
    }
    if let (Some(verify_code), Some(uuid)) = (item.verify_code.clone(), item.uuid.clone()) {
        if verify_code.is_empty() || uuid.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("验证不能为空或者参数错误".to_string()));
        }

        // 查询缓存内的验证码
        let cache_captch = CONTEXT.cache_service.get_string(&format!("captch:cache_{}", uuid.as_str())).await.unwrap_or_default();
        if cache_captch.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("验证码已过期或者不存在".to_string()));
        }

        // 比较验证码
        if cache_captch != verify_code {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("验证码不正确".to_string()));
        }

        // 删除验证码缓存
        CONTEXT.cache_service.del(&format!("captch:cache_{}", uuid.as_str())).await.unwrap_or_default();
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("验证不能为空或者参数错误".to_string()));
    }
    
    let user_result = admin_service::select_by_username(&item.username).await;
    match user_result {
        Ok(u) => {
            match u {
                None => {
                    // 记录登录日志
                    let method = request.method().to_string();
                    let _ = system_log_service::save_system_log(&request, Some("用户登录失败".to_string()), Some(0),Some("system_admin_controller::login".to_string()), Some(method.to_string()),Some(1)).await;
                    
                    HttpResponse::Ok().json(ResVO::<String>::error_msg("用户不存在".to_string()))
                }
                Some(user_info) => {
                    let valid = verify(&item.password.clone().unwrap_or_default(), &user_info.password.clone().unwrap_or_default()).unwrap_or_default();
                    if !valid {
                        return HttpResponse::Ok().json(ResVO::<String>::error_msg("密码不正确".to_string()));
                    }
                    //判断是否是管理员
                    let is_admin = user_info.user_type == Option::from(1);

                    //查询出用户菜单权限
                    let result = menus_service::get_router_tree(&is_admin, &user_info.id).await;
                    let routers: Vec<Router> = match result {
                        Ok(v) => v.clone(),
                        Err(err) => {
                            return HttpResponse::Ok().json(ResVO::<String>::error_msg("查询菜单异常,".to_string() + &err.to_string()));
                        }
                    };

                    if routers.is_empty() {
                        return HttpResponse::Ok().json(ResVO::<String>::error_msg("用户没有分配角色或者菜单,不能登录".to_string()));
                    };
                    //查询用户所在权限组
                    let admin_role: Vec<String> = match menus_service::query_btn_menu(&is_admin, &user_info.id).await {
                        Ok(role_list) => role_list.clone(),
                        Err(err) => {
                            return HttpResponse::Ok().json(ResVO::<String>::error_msg(format!("权限组查询失败, {}", err)));
                        }
                    };
                    let setting = Settings::get();
                    return match JWTToken::new(user_info.id.clone(), user_info.user_name.clone(), admin_role.clone()).create_token(&setting.server.jwt_secret) {
                        Ok(token) => {

                            // 记录登录日志
                            let method = request.method().to_string();
                            system_log_service::save_system_log(&request, Some("用户登录".to_string()), Some(0),Some("system_admin_controller::login".to_string()), Some(method.to_string()),Some(1)).await.unwrap_or_default();

                            let update_user:SystemAdmin = UpdateLoginRequest {
                                id: user_info.id,
                                login_ip: Option::from(request.connection_info().realip_remote_addr().unwrap_or_default().to_string()),
                                login_date: DateTime::now().into(),
                            }.into();
                            admin_service::update_admin(update_user).await.unwrap_or_default();

                            let user_login = UserLoginResponse {
                                token,
                                user_info: user_info.clone(),
                                menu_list: routers,
                                permissions: admin_role.clone(),
                                username: user_info.user_name.clone(),
                            };
                            HttpResponse::Ok().json(ResVO::ok_with_data(user_login))
                        }
                        Err(err) => {
                            let er = match err {
                                WhoUnfollowedError::JwtTokenError(s) => { s }
                                _ => "no math error".to_string()
                            };
                            HttpResponse::Ok().json(ResVO::<String>::error_msg(er))
                        }
                    }
                }
            }
        }

        Err(_err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg("查询用户异常".to_string()))
        }
    }
}

// 删除用户信息
#[delete("/system/user/delete")]
pub async fn user_delete(item: web::Json<BathIdRequest>) -> HttpResponse {
    if let Some(ids_vec) = item.ids.clone() {
        for id_opt in ids_vec.iter() {
            if let Some(id) = id_opt {
                if id == "1" {
                    return HttpResponse::Ok().json(ResVO::<String>::error_msg("含有不能删除的超级管理员账户".to_string()));
                }
            }
        }

        if ids_vec.is_empty() {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()));
        }

        let result = admin_service::delete_in_column(&ids_vec).await;
        HttpResponse::Ok().json(&ResVO::<u64>::handle_result(result))
    } else {
        HttpResponse::Ok().json(ResVO::<String>::error_msg("删除的ID不能为空".to_string()))
    }
}

#[put("/system/update_user_role")]
pub async fn update_user_role(item: web::Json<UpdateAdminRoleRequest>) -> HttpResponse {
    //log::info!("update_user_role params: {:?}", item);
    let user_role = item.0;
    let role = role_service::update_user_role(user_role).await;
    return role;
}


// 更新用户信息
#[put("/system/admin/update")]
pub async fn admin_update(item: web::Json<AdminUpdateRequest>) -> HttpResponse {
    let item = item.0;
    if let Some(id) = item.id {
        if id == 1 && item.status == Option::from(0) {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("超级管理员不能禁用".to_string()));
        }
    } else {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("更新的用户id不能为空".to_string()));
    }
    if admin_service::find_by_name_unique(&item.user_name, &item.id).await.unwrap_or_default(){
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("用户名已存在".to_string()));
    }
    if admin_service::find_by_mobile_unique(&item.mobile, &item.id).await.unwrap_or_default(){
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("手机号已存在".to_string()));
    }
    if item.email.is_some() {
        if admin_service::find_by_email_unique(&item.email, &item.id).await.unwrap_or_default(){
            return HttpResponse::Ok().json(ResVO::<String>::error_msg("邮箱已存在".to_string()));
        }
    }
    if admin_service::find_by_nick_name_unique(&item.nick_name, &item.id).await.unwrap_or_default(){
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("昵称已存在".to_string()));
    }
    
    let result = admin_service::get_by_detail(&item.id).await;
    match result {
        Ok(Some(_)) => {
            depts_service::batch_update_dept(&item.dept_ids, &item.id).await.unwrap_or_default();
            post_service::batch_update_post(&item.post_ids, &item.id).await.unwrap_or_default();
            role_service::batch_update_role(&item.role_ids, &item.id).await.unwrap_or_default();
            let admin: SystemAdmin = item.into();
            let result = admin_service::update_admin(admin).await;
            HttpResponse::Ok().json(ResVO::<u64>::handle_result(result))
        }
        Ok(None) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg("用户不存在".to_string()))
        }
        Err(err) => {
            log::error!("更新用户信息异常: {:?}", err);
            HttpResponse::Ok().json(ResVO::<String>::error_msg("更新用户信息异常".to_string()))
        }
    }
}

// 更新用户密码
#[put("/system/admin/update_password")]
pub async fn update_admin_password(req: HttpRequest, item: web::Json<UpdateAdminPasswordRequest>) -> HttpResponse {
    let mut admin_pwd:SystemAdmin = item.0.into();
    if admin_pwd.password.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("密码不能为空".to_string()));
    }
    //获取当前用户id
    let admin_token:JWTToken = get_user(req).unwrap_or_default();
    //这里是为了防止账户未退出时被人修改里密码，但是漏洞是可修改其他管理员后登录再改这个密码
    if admin_token.id == admin_pwd.id {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("不可通过列表页面修改当前用户密码".to_string()));
    }
    
    let sys_admin_result = admin_service::get_by_detail(&admin_pwd.id).await;
    return match sys_admin_result {
        Ok(admin_result) => {
            match admin_result {
                None => {
                    HttpResponse::Ok().json(ResVO::<String>::error_msg("用户不存在".to_string()))
                }
                Some(_) => {
                    let hashed = hash(admin_pwd.password.clone().unwrap_or_default(), DEFAULT_COST).unwrap_or_default();
                    admin_pwd.password = Option::from(hashed);
                    let result = admin_service::update_by_password(&admin_pwd).await;
                    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result))
                }
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

// 更新用户密码
#[put("/system/admin/update_my_password")]
pub async fn update_my_password(req: HttpRequest, item: web::Json<UpdateAdminPasswordRequest>) -> HttpResponse {
    let user_pwd = item.0;
    if user_pwd.password.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("密码不能为空".to_string()));
    }
    //获取当前用户id
    let admin_token:JWTToken = get_user(req).unwrap_or_default();
    let sys_user_result = admin_service::get_by_detail(&admin_token.id).await;
    return match sys_user_result {
        Ok(user_result) => {
            match user_result {
                None => {
                    HttpResponse::Ok().json(ResVO::<String>::error_msg("用户不存在".to_string()))
                }
                Some(mut user) => {
                    if user.password == user_pwd.re_password {
                        user.password = user_pwd.re_password;
                        let result = admin_service::update_by_password(&user).await;
                        return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result))
                    } else {
                        HttpResponse::Ok().json(ResVO::<String>::error_msg("旧密码不正确".to_string()))
                    }
                }
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

#[put("/system/admin/update-status")]
pub async fn update_admin_status(item: web::Json<UpdateAdminStatusRequest>) -> HttpResponse {
    let admin_status = item.0;
    if admin_status.id.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("用户id不能为空".to_string()));
    }
    if admin_status.id == Option::from(1) {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("超级管理员不能禁用".to_string()));
    }
    let result = admin_service::update_by_status(admin_status).await;
    return HttpResponse::Ok().json(ResVO::<u64>::handle_result(result))
}



#[get("/system/admin/detail/{id}")]
pub async fn get_user_detail(item: web::Path<InfoId>) -> HttpResponse {
    if item.id.is_none() {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("角色id不能为空".to_string()));
    }
    return match admin_service::get_by_detail(&item.id).await {
        Ok(user_op) => match user_op {
            Some(admin) => {
                let mut admin_detail: SystemAdminVO = admin.into();
                // 查询用户关联的角色
                let result_roles = role_service::select_by_admin_id(&admin_detail.id).await.unwrap_or_default();
                let mut role_data: Vec<Option<u64>> = Vec::new();
                for role in result_roles {
                    role_data.push(role.id);
                }
                admin_detail.role_ids = Option::from(role_data);
                // 查询用户关联的部门
                let result_depts = depts_service::select_by_admin_id(&admin_detail.id).await.unwrap_or_default();
                let mut dept_data: Vec<Option<u64>> = Vec::new();
                for dept in result_depts {
                    dept_data.push(dept.id);
                }
                admin_detail.dept_ids = Option::from(dept_data);
                // 查询用户关联的岗位
                let result_posts = post_service::select_by_admin_id(&admin_detail.id).await.unwrap_or_default();
                let mut post_data: Vec<Option<u64>> = Vec::new();
                for post in result_posts {
                    post_data.push(post.id);
                }
                admin_detail.post_ids = Option::from(post_data);

                HttpResponse::Ok().json(ResVO::ok_with_data(admin_detail))
            }
            None => {
                HttpResponse::Ok().json(ResVO::<String>::error_msg("用户信息不存在".to_string()))
            }
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

#[get("/system/user/params")]
pub async fn get_user_params() -> HttpResponse {
    return HttpResponse::Ok().json(ResVO::ok_with_data(RoleAndPostVO {
        role_list: role_service::select_all().await.unwrap_or_default(),
        post_list: post_service::select_all().await.unwrap_or_default(),
    }));
}

// 查询用户列表
#[get("/system/admin/list")]
#[protect("admin:list:show")]
pub async fn admin_list(item: web::Query<UserListRequest>) -> HttpResponse {
    let admin_request = item.0;
    let result = admin_service::select_by_page(admin_request).await;
    return match result {
        Ok(page) => {
            //获取名字id列表
            let id_list: Vec<Option<u64>> = page.records.iter().map(|data| data.id).collect();
            let result_role_name = role_service::select_by_ids(&id_list).await;
            let result_dept_name = depts_service::select_by_ids(&id_list).await;
            let mut list_data: Vec<AdminListVO> = Vec::new();
            for user in page.records {
                let mut role_data: Vec<RoleNameDTO> = Vec::new();
                match result_role_name {
                    Ok(ref role_list) => {
                        for role_entity in role_list {
                            if role_entity.admin_id == user.id {
                                role_data.push(RoleNameDTO{ role_name: role_entity.role_name.clone() });
                            }
                        }
                    }
                    Err(_) => {}
                }
                
                let mut depts_data: Vec<DeptsNameDTO> = Vec::new();
                match result_dept_name {
                    Ok(ref dept_list) => {
                        for dept_entity in dept_list {
                            if dept_entity.admin_id == user.id {
                                depts_data.push(DeptsNameDTO {dept_name: dept_entity.dept_name.clone()});
                            }
                        }
                    }
                    Err(_) => {}
                }

                list_data.push(AdminListVO {
                    id: user.id,
                    sort: user.sort,
                    status: user.status,
                    mobile: user.mobile,
                    user_name: user.user_name,
                    nick_name: user.nick_name,
                    roles: Option::from(role_data),
                    depts: Option::from(depts_data),
                    remark: user.remark,
                    create_time: user.create_time.clone().map(|t| t.format("YYYY-MM-DD hh:mm:ss")).unwrap_or_default(),
                })
            }
            let page_data = ResultPage {
                current_page: page.page_no,
                list: list_data,
                total: page.total,
            };
            HttpResponse::Ok().json(ok_result_page(page_data))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(
                "查询管理员列表异常,".to_string() + &err.to_string()
            ))
        }
    }
}


// 退出登录
#[get("/system/logout")]
pub async fn logout() -> HttpResponse {
    return HttpResponse::Ok().json(ResVO::<String>::ok_msg("退出成功".to_string()))
}