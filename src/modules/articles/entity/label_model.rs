use rbatis::rbdc::{DateTime};
use serde::{Deserialize, Serialize};
use crate::modules::articles::entity::label_entity::Label;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabelSaveRequest {
    // 短网址
    pub short_url: String,
    // 标签名称
    pub title: String,
    // 话题描述
    pub content: Option<String>,
    // 话题图片
    pub label_pic: Option<String>,
    // 标签使用数量
    pub count_topic: i32,
    //1为推荐
    pub isrecommend: i8,
    // 添加时间
    pub create_time: Option<DateTime>,
    // 0未审核，1：正常显示；2：隐藏不显示
    pub status: i8,
}

impl From<LabelSaveRequest> for Label {
    fn from(arg: LabelSaveRequest) -> Self {
        Self {
            id: 0,
            short_url: arg.short_url,
            title: arg.title,
            content: arg.content,
            label_pic: arg.label_pic,
            count_view: 0,
            label_lock: 0,
            count_topic: arg.count_topic,
            count_group: 0,
            count_follow: 0,
            isrecommend: arg.isrecommend,
            create_time: Option::from(DateTime::now()),
            status: arg.status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabelUpdateRequest {
    pub id: u64,
    // 短网址
    pub short_url: String,
    // 标签名称
    pub title: String,
    // 话题描述
    pub content: Option<String>,
    // 话题图片
    pub label_pic: Option<String>,
    // 标签使用数量
    pub count_topic: i32,
    //1为推荐
    pub isrecommend: i8,
    // 添加时间
    pub create_time: Option<DateTime>,
    // 0未审核，1：正常显示；2：隐藏不显示
    pub status: i8,
}

impl From<LabelUpdateRequest> for Label {
    fn from(arg: LabelUpdateRequest) -> Self {
        Self {
            id: arg.id,
            short_url: arg.short_url,
            title: arg.title,
            content: arg.content,
            label_pic: arg.label_pic,
            count_view: 0,
            label_lock: 0,
            count_topic: arg.count_topic,
            count_group: 0,
            count_follow: 0,
            isrecommend: arg.isrecommend,
            create_time: Option::from(DateTime::now()),
            status: arg.status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabelPageRequest {
    pub title: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub status: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabelPageBO {
    pub title: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub status: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

impl From<LabelPageRequest> for LabelPageBO {
    fn from(req: LabelPageRequest) -> Self {
        Self {
            title: req.title,
            status: req.status,
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}