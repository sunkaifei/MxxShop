use rbatis::{impl_select_page};

use crate::modules::system::entity::dict_data_entity::DictData;
use crate::modules::system::entity::dict_data_model::QueryDictDataPageBO;
use crate::modules::system::entity::dict_type_entity::DictType;
use crate::modules::system::entity::dict_type_model::{DictTypePageBO};

//增删改查字典数据
rbatis::crud!(DictData {}, "fly_system_dict_data");

//字典数据分页查询
impl_select_page!(DictData{select_page(dto: &QueryDictDataPageBO) =>"
     trim end=' where ':
       ` where `
       trim ' and ':
         if dto.dict_label != null && dto.dict_label != '':
           ` and dict_label like '%${dto.dict_label}%'`
         if dto.dict_value != null && dto.dict_value != '':
           ` and dict_value = #{dto.dict_value} `
         choose:
           when dto.status == 0:
             ` and status >= 0 `
           when dto.status == 1:
             ` and status = 0 `
           when dto.status == 2:
             ` and status = 1 `
     if !sql.contains('count'):
        ` order by id desc `"},"fly_system_dict_data");

//增删改查字典数据类型
rbatis::crud!(DictType {}, "fly_system_dict_type");

impl_select_page!(DictType{select_page(dto: &DictTypePageBO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.dict_name != null && dto.dict_name != '':
          ` and dict_name like '%${dto.dict_name}%'`
        if dto.dict_type != null && dto.dict_type != '':
          ` and dict_type = #{dto.dict_type} `
        choose:
          when dto.status == 0:
            ` and status >= 0 `
          when dto.status == 1:
            ` and status = 0 `
          when dto.status == 2:
            ` and status = 1 `
    if !sql.contains('count'):
      ` order by id desc `"},"fly_system_dict_type");


