use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub id: u64,
    // 短网址
    pub short_url: String,
    // 标签名称
    pub title: String,
    // 话题描述
    pub content: Option<String>,
    // 话题图片
    pub label_pic: Option<String>,
    // 访问统计
    pub count_view: i32,
    // 话题是否锁定 1 锁定 0 未锁定
    pub label_lock: i8,
    // 标签使用数量
    pub count_topic: i32,

    pub count_group: i32,
    // 关注数
    pub count_follow: i32,
    //1为推荐
    pub isrecommend: i8,
    // 添加时间
    pub create_time: Option<DateTime>,
    // 0未审核，1：正常显示；2：隐藏不显示
    pub status: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabelMerge {
    ///标签ID
    label_id: u64,
    ///标签信息ID
    info_id: u64,
    ///标签信息类型 1 文章 2 群组
    info_type: i8,
}