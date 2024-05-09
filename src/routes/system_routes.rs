//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use std::collections::HashSet;

use actix_web::{Error, Result, web};
use actix_web::dev::ServiceRequest;
use actix_web_grants::GrantsMiddleware;

use crate::core::permission::jwt_util::JWTToken;
use crate::modules::articles::controller::admin::{article_admin_controller, article_category_admin_controller};
use crate::modules::system::controller::admin::{config_admin_controller, dept_admin_controller, ip_admin_controller, menu_admin_controller, role_admin_controller, system_admin_controller, system_dict_controller};
use crate::utils::settings::Settings;

async fn extract(req: &ServiceRequest) -> Result<HashSet<String>, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .map(|v| v.to_str().unwrap_or_default().to_string())
        .unwrap_or_default()
        .split("Bearer ")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap_or_default()
        .to_string();
    let setting = Settings::get();
    let jwt_token_e = JWTToken::verify(&setting.server.jwt_secret, &token);

    match jwt_token_e {
        Ok(data) => {
            let set: HashSet<String> = data.permissions.into_iter().collect();
            return Ok(set)
        },
        Err(_err) => {
        }
    };

    Ok(HashSet::from(["".to_string()]))
}

// `Eq` and `Hash` is required
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Role {
    Admin,
    Manager,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(GrantsMiddleware::with_extractor(extract))                                            // 添加身份验证中间件
            .service(system_admin_controller::save_admin) // 添加管理员
            .service(system_admin_controller::user_delete)// 删除管理员
            .service(system_admin_controller::update_user_role)// 更新用户角色
            .service(system_admin_controller::user_update)
            .service(system_admin_controller::update_admin_password)//更新管理员密码
            .service(system_admin_controller::get_user_detail)
            .service(system_admin_controller::login)   // 管理员登录
            .service(system_admin_controller::admin_list)// 管理员列表
            .service(system_admin_controller::logout)      //退出登录
            .service(role_admin_controller::role_save)// 角色保存         
            .service(role_admin_controller::bath_delete_role) // 角色删除
            //.service(web::resource("/system/role/update_role_menu").route(web::put().to(role_admin_controller::update_role_menu)))// 更新角色菜单
            .service(role_admin_controller::update_role)// 角色更新
            .service(role_admin_controller::role_list)// 角色列表
            .service(role_admin_controller::get_role_detail)// 角色详情
            .service(menu_admin_controller::menu_list)// 菜单列表
            .service(menu_admin_controller::menu_save)// 菜单保存
            .service(menu_admin_controller::menu_delete) // 菜单删除
            .service(menu_admin_controller::menu_update)// 菜单更新
            .service(menu_admin_controller::menu_detail)
            .service(menu_admin_controller::get_menu_params)
            .service(menu_admin_controller::get_user_menu)//重新获取用户菜单权限信息
            .service(system_dict_controller::save_dict_type)
            .service(system_dict_controller::batch_delete_type)
            .service(system_dict_controller::update_dict_type)
            .service(system_dict_controller::get_dict_type_page)
            .service(system_dict_controller::get_dict_type_detail)
            .service(system_dict_controller::save_dict_data)
            .service(system_dict_controller::batch_delete_data)
            .service(system_dict_controller::update_dict_data)
            .service(system_dict_controller::get_dict_data_list)
            //.service(web::resource("/system/dict/data/getDictData").route(web::get().to(system_dict_controller::get_dict_data_list)))
            .service(system_dict_controller::get_dict_data_detail)
            .service(dept_admin_controller::dept_save)// 部门保存
            .service(dept_admin_controller::dept_update)//更新部门
            .service(dept_admin_controller::query_dept_tree)// 查询部门树
            .service(dept_admin_controller::get_dept_detail)
            .service(dept_admin_controller::dept_list)// 部门列表
            .service(dept_admin_controller::dept_batch_delete)// 批量部门删除
            .service(config_admin_controller::get_config_page)
            .service(config_admin_controller::get_config_detail)
            .service(article_admin_controller::get_article_list)
            .service(article_category_admin_controller::category_list_tree)
            .service(ip_admin_controller::ip_address_page)
            //.service(web::resource("/system/ip/batch_delete").route(web::delete().to(ip_admin_controller::batch_delete_ip)))
            //.service(web::resource("/system/ip/update").route(web::put().to(ip_admin_controller::update_ip)))
            //.service(web::resource("/system/ip/detail/{id}").route(web::get().to(ip_admin_controller::get_ip_detail)))
    );
}
