//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use chrono::NaiveDateTime;
use rbatis::rbdc::{DateTime, Decimal};

#[derive(Debug)]
pub struct MxxCart {
    /// 自增id
    pub id: Option<u64>,
    /// 用户id
    pub user_id: Option<u64>,
    /// 商品id
    pub product_id: Option<u64>,
    /// 标题
    pub title: Option<String>,
    /// 封面图片
    pub images: Option<String>,
    /// 原价
    pub original_price: Option<Decimal>,
    /// 销售价格
    pub price: Option<Decimal>,
    /// 购买数量
    pub stock: Option<u32>,
    /// 规格
    pub spec: Option<String>,
    /// 添加时间
    pub create_time: Option<DateTime>,
    /// 更新时间
    pub update_time: Option<DateTime>,
}

