//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{crud, impl_select_page};

use crate::modules::system::entity::language_entity::{LangType, LangCountry, LangCode};
use crate::modules::system::entity::language_model::{QueryTypePageDTO, QueryCountryPageDTO, QueryCodePageDTO};

// 语言类型增删改查
crud!(LangType {}, "mxx_lang_type");

// 分页查询
impl_select_page!(LangType{select_page(dto: &QueryTypePageDTO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.language_name != null && dto.language_name != '':
          ` and language_name = #{dto.language_name} `
        choose:
          when dto.status == 0:
            ` and status >= 0 `
          when dto.status == 1:
            ` and status = 0 `
          when dto.status == 2:
            ` and status = 1 `
        choose:
          when dto.is_del == 0:
            ` and is_del >= 0 `
          when dto.status == 1:
            ` and is_del = 0 `
          when dto.is_del == 2:
            ` and status = 1 `
    if !sql.contains('count'):
      ` order by id desc `"},"mxx_lang_type");



// 语言国家类型增删改查
crud!(LangCountry {}, "mxx_lang_country");

// 分页查询
impl_select_page!(LangCountry{select_page(dto: &QueryCountryPageDTO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.type_id != null && dto.type_id != '':
          ` and type_id = #{dto.type_id} `
        if dto.code != null && dto.code != '':
          ` and code = #{dto.code} `
        if dto.country_name != null && dto.country_name != '':
          ` and country_name like concat('%', #{dto.country_name}, '%') `
    if !sql.contains('count'):
      ` order by id desc `"},"mxx_lang_country");


// 语言code表增删改查
crud!(LangCode {}, "mxx_lang_code");

// 分页查询
impl_select_page!(LangCode{select_page(dto: &QueryCodePageDTO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.type_id != null && dto.type_id != '':
          ` and type_id = #{dto.type_id} `
        if dto.code != null && dto.code != '':
          ` and code = #{dto.code} `
        if dto.is_admin != null && dto.is_admin != '':
          ` and is_admin = #{dto.is_admin} `
    if !sql.contains('count'):
      ` order by id desc `"},"mxx_lang_code");
