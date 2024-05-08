use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    ///评论ID
    pub id: u64,
    ///上级评论ID
    pub refer_id: u64,
    ///文章ID
    pub article_id: u64,
    ///用户ID
    pub user_id: u64,
    ///回复内容
    pub content: String,
    ///被评论数量
    pub count_comment: i32,
    ///支持数
    pub count_digg: i32,
    ///反对次数
    pub count_burys: i32,
    ///排序权重
    pub weight: f64,
    ///0公开1不公开（仅自己和发帖者可看）
    pub ispublic: i8,
    ///楼层数量
    pub storey: i32,
    ///创建时间
    pub create_time: Option<DateTime>,
    ///更新时间
    pub update_time: Option<DateTime>,
    ///0未审核，1审核，2未通过
    pub status: i8,
    ///'0不删除1删除
    pub deleted: i8,
}