use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::types::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Articles {
    //文章ID
    pub id: u64,
    //短网址
    pub short_url: Option<String>,
    //文章分类ID
    pub category_id: Option<u64>,
    //所属所有的父级至子级分类
    pub category_ids: Option<String>,
    //用户ID
    pub user_id: u64,
    //发布用户类型，1管理员，0前台用户
    pub user_type: i8,
    //文章标题
    pub title: Option<String>,
    //简短标题
    pub short_title: Option<String>,
    //文章主图
    pub title_image: Option<String>,
    //文章作者
    pub author: Option<String>,
    //原文链接
    pub original_link: Option<String>,
    //文章摘要
    pub description: Option<String>,
    //文章内容
    pub content: Option<String>,
    //回复统计
    pub count_comment: Option<u64>,
    //帖子展示数
    pub count_view: Option<u64>,
    //喜欢数
    pub count_love: Option<u64>,
    //支持数
    pub count_digg: Option<u64>,
    //反对次数
    pub count_burys: Option<u64>,
    //关注话题统计数
    pub count_follow: Option<u64>,
    //排序权重
    pub weight: Option<Decimal>,
    //是否置顶
    pub istop: i8,
    //是否关闭帖子
    pub isclose: i8,
    //是否允许评论
    pub iscomment: i8,
    //是否评论后显示内容
    pub iscommentshow: i8,
    //是否精华帖子
    pub isposts: i8,
    //0不审核，1审核，2未通过
    pub isaudit: i8,
    //0不删除1删除
    pub deleted: i8,
    //1为推荐
    pub isrecommend: i8,
    //0未审核，1审核，2未通过
    pub status: i8,
    //创建时间
    pub create_time: Option<DateTime>,
    //更新时间
    pub update_time: Option<DateTime>,
}


