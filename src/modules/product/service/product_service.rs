//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use crate::pool;
use rbatis::{Page, PageRequest};
use crate::core::errors::error::Result;
use crate::modules::product::entity::product_entity::Product;
use crate::modules::product::entity::product_model::{ProductPageBO, ProductSaveRequest};
use crate::modules::product::mapper::product_mapper;

///发布产品信息
pub async fn save_product(item: ProductSaveRequest) -> Result<u64>{
    let entity: Product = item.into();
    // 启动一个事务
    let mut tx = pool!().acquire_begin().await?;
    let result = Product::insert(&tx, &entity).await?;
    return if result.rows_affected > 0 {
        tx.commit().await?;
        tx.rollback().await?;
        Ok(result.rows_affected)
    }else{
        Ok(0)
    }
}

///批量删除产品信息
pub async fn batch_product_delete(ids: Vec<String>) -> Result<u64>{
    let result = Product::delete_in_column(pool!(), "id", &ids).await?;
    Ok(result.rows_affected)
}

///更新产品信息
pub async fn update_product(item: Product) -> Result<u64>{
    let result = Product::update_by_column(pool!(), &item, "id").await?;
    Ok(result.rows_affected)
}

/// 根据名称查询产品标题是否唯一

pub async fn find_by_name_unique(product_name: &Option<String>, id: &Option<u64>) -> Result<bool>{
    let result = product_mapper::find_by_name_unique(pool!(), product_name, id).await?;
    return if result > 0 {
        Ok(true)
    }else{
        Ok(false)
    }
}

///根据id查询产品详情
pub async fn get_by_detail(id: Option<u64>) -> rbatis::Result<Option<Product>>{
    Ok(Product::select_by_column(pool!(),"id", id).await?
        .into_iter()
        .next())
}

///查询产品列表
pub async fn select_by_page(item: ProductPageBO) -> rbatis::Result<Page<Product>>{
    let page_req = &PageRequest::new(1, 10);
    let result = Product::select_by_page(pool!(), page_req, &item).await;
    return result;
}