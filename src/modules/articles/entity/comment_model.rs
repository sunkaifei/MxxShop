use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentSaveRequest {
    ///上级评论ID
    pub refer_id: u64,
    // 文章ID
    pub article_id: u64,
    ///用户ID
    pub user_id: u64,
    // 回复内容
    pub content: String,
    ///0公开 1不公开（仅自己和发帖者可看）
    pub ispublic: i8,
    ///楼层数量
    pub storey: i32,
    ///创建时间
    pub create_time: Option<DateTime>,
    ///更新时间
    pub update_time: Option<DateTime>,
    ///0未审核，1审核，2未通过
    pub status: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentPageRequest {
    pub title: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub status: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentPageBO {
    pub title: Option<String>,
    // 状态查询（0所有，1查询为0的数据，2查询为1的数据）
    pub status: Option<i8>,
    // 当前页码数
    pub page_num: u64,
    // 每页条数
    pub page_size: u64,
}

impl From<CommentPageRequest> for CommentPageBO {
    fn from(req: CommentPageRequest) -> Self {
        Self {
            title: req.title,
            status: req.status,
            page_num: req.page_num,
            page_size: req.page_size,
        }
    }
}