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
use crate::modules::product::entity::product_entity::Product;
use crate::utils::string_utils::{serialize_option_u64_to_string,deserialize_string_to_u64,deserialize_string_to_i8,deserialize_string_to_i32};


/// 商品保存请求
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductSaveRequest {
    /// 语言id
    #[serde(deserialize_with = "deserialize_string_to_u64")]
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
    #[serde(deserialize_with = "deserialize_string_to_u64")]
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
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub sort: Option<i32>,
    /// 销量
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub sales: Option<i32>,
    /// 库存
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub stock: Option<i32>,
    /// 状态（0：未上架，1：上架）
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_show: Option<i8>,
    /// 是否热卖
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_hot: Option<i8>,
    /// 是否优惠
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_benefit: Option<i8>,
    /// 是否精品
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_best: Option<i8>,
    /// 是否新品
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_new: Option<i8>,
    /// 商品是否是虚拟商品
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_virtual: Option<i8>,
    /// 虚拟商品类型
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub virtual_type: Option<i8>,
    /// 是否包邮
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_postage: Option<i8>,
    /// 积分
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub give_integral: Option<i32>,
}

impl From<ProductSaveRequest> for Product {
    fn from(req: ProductSaveRequest) -> Self {
        Self {
            id: None,
            lang_id: req.lang_id,
            image: req.image,
            recommend_image: req.recommend_image,
            slider_image: req.slider_image,
            product_name: req.product_name,
            product_info: req.product_info,
            keyword: req.keyword,
            bar_code: req.bar_code,
            cate_id: req.cate_id,
            price: req.price,
            vip_price: req.vip_price,
            ot_price: req.ot_price,
            postage: req.postage,
            unit_name: req.unit_name,
            sort: req.sort,
            sales: req.sales,
            stock: req.stock,
            is_show: req.is_show,
            is_hot: req.is_hot,
            is_benefit: req.is_benefit,
            is_best: req.is_best,
            is_new: req.is_new,
            is_virtual: req.is_virtual,
            virtual_type: req.virtual_type,
            is_postage: req.is_postage,
            is_del: None,
            give_integral: req.give_integral,
            create_time: Option::from(DateTime::now()),
        }
    }

}

/// 商品更新请求
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductUpdateRequest {
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
    /// 积分
    pub give_integral: Option<i32>,
}

impl From<ProductUpdateRequest> for Product {
    fn from(req: ProductUpdateRequest) -> Self {
        Self {
            id: req.id,
            lang_id: req.lang_id,
            image: req.image,
            recommend_image: req.recommend_image,
            slider_image: req.slider_image,
            product_name: req.product_name,
            product_info: req.product_info,
            keyword: req.keyword,
            bar_code: req.bar_code,
            cate_id: req.cate_id,
            price: req.price,
            vip_price: req.vip_price,
            ot_price: req.ot_price,
            postage: req.postage,
            unit_name: req.unit_name,
            sort: req.sort,
            sales: req.sales,
            stock: req.stock,
            is_show: req.is_show,
            is_hot: req.is_hot,
            is_benefit: req.is_benefit,
            is_best: req.is_best,
            is_new: req.is_new,
            is_virtual: req.is_virtual,
            virtual_type: req.virtual_type,
            is_postage: req.is_postage,
            is_del: None,
            give_integral: req.give_integral,
            create_time: None,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductPageRequest {
    /// 商品名称
    pub product_name: Option<String>,
    /// 分类id
    pub cate_id: Option<u64>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductPageBO {
    /// 商品名称
    pub product_name: Option<String>,
    /// 分类id
    pub cate_id: Option<u64>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

impl From<ProductPageRequest> for ProductPageBO {
    fn from(req: ProductPageRequest) -> Self {
        Self {
            product_name: req.product_name,
            cate_id: req.cate_id,
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductListVO {
    /// 商品id
    pub id: Option<u64>,
    /// 语言id
    pub lang_id: Option<u64>,
    /// 商品图片
    pub image: Option<String>,
    /// 商品名称
    pub product_name: Option<String>,
    /// 分类id
    pub cate_id: Option<u64>,
    /// 商品价格
    pub price: Option<Decimal>,
    /// 会员价格
    pub vip_price: Option<Decimal>,
    /// 市场价
    pub ot_price: Option<Decimal>,
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
    /// 添加时间
    pub create_time: Option<String>,
}