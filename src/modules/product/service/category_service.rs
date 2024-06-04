//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use crate::core::errors::error::{Error, Result};
use crate::modules::product::entity::category_entity::ProductCategory;
use crate::modules::product::mapper::category_mapper;
use crate::pool;

///保存产品分类
pub async fn save_category(item: ProductCategory) -> Result<u64> {
    let rows = ProductCategory::insert(pool!(), &item).await?;
    Ok(rows.rows_affected)
}

///批量删除产品分类
pub async fn batch_delete_in_column(ids_vec: Vec<Option<String>>) -> Result<u64> {
    return if ids_vec.is_empty() {
        Err(Error::from("删除的ID不能为空".to_string()))
    } else {
        let result = ProductCategory::delete_in_column(pool!(), "id", &ids_vec).await?;
        Ok(result.rows_affected)
    }
}

///按id更新产品分类
pub async fn update_category(item: ProductCategory) -> Result<u64> {
    let rows = ProductCategory::update_by_column(pool!(), &item, "id").await?;
    Ok(rows.rows_affected)
}

///查询同父级类目下分类名称是否唯一
pub async fn find_by_name(category_name: &Option<String>, parent_id: &Option<u64>, id: &Option<u64>) -> Result<bool> {
    let result = category_mapper::find_by_name_unique(pool!(), category_name, parent_id, id).await?;
    return if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

///按id查询产品分类详情
pub async fn select_by_id(id: Option<u64>) -> rbatis::Result<Option<ProductCategory>> {
    Ok(ProductCategory::select_by_column(pool!(),"id", id).await?
        .into_iter()
        .next())
}

///查询所有产品分类列表
pub async fn select_all_list() -> rbatis::Result<Vec<ProductCategory>> {
    let result = category_mapper::select_all_list(pool!()).await;
    return result;
}
