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
use crate::modules::product::entity::product_entity::Product;
use crate::modules::product::entity::product_model::{ProductPageBO};

pub async fn select_by_page(item: ProductPageBO) -> rbatis::Result<Page<Product>>{
    let page_req = &PageRequest::new(1, 10);
    let result = Product::select_by_page(pool!(), page_req, &item).await;
    return result;
}