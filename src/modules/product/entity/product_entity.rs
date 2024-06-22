//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::rbdc::{DateTime, Decimal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    /// 商品id
    pub id: Option<u64>,
    /// 语言id
    pub lang_id: Option<u64>,
    /// 商品图片
    pub image: Option<String>,
    /// 推荐图
    pub recommend_image: Option<String>,
    /// 轮播图
    pub slider_image: Option<String>,
    /// 商品名称
    pub product_name: Option<String>,
    /// 商品简介
    pub product_info: Option<String>,
    /// 关键字
    pub keyword: Option<String>,
    /// 商品条码（一维码）
    pub bar_code: Option<String>,
    /// 分类id
    pub cate_id: Option<u64>,
    /// 商品价格
    pub price: Option<Decimal>,
    /// 会员价格
    pub vip_price: Option<Decimal>,
    /// 市场价
    pub ot_price: Option<Decimal>,
    /// 邮费
    pub postage: Option<Decimal>,
    /// 单位名
    pub unit_name: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 销量
    pub sales: Option<i32>,
    /// 库存
    pub stock: Option<i32>,
    /// 状态（0：未上架，1：上架）
    pub is_show: Option<i8>,
    /// 是否热卖
    pub is_hot: Option<i8>,
    /// 是否优惠
    pub is_benefit: Option<i8>,
    /// 是否精品
    pub is_best: Option<i8>,
    /// 是否新品
    pub is_new: Option<i8>,
    /// 商品是否是虚拟商品
    pub is_virtual: Option<i8>,
    /// 虚拟商品类型
    pub virtual_type: Option<i8>,
    /// 是否包邮
    pub is_postage: Option<i8>,
    /// 是否删除
    pub is_del: Option<i8>,
    /// 积分
    pub give_integral: Option<i32>,
    /// 添加时间
    pub create_time: Option<DateTime>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Favorites {
    /// 自增id
    pub id: Option<u64>,
    /// 商品id
    pub product_id: Option<u64>,
    /// 用户id
    pub user_id: Option<u64>,
    /// 添加时间
    pub create_time: Option<DateTime>,
}