use std::str::FromStr;

use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::types::Decimal;
use serde::{Deserialize, Serialize};

use crate::modules::articles::entity::article_entity::Articles;
use crate::modules::user::entity::user_entity::User;
use crate::utils::string_utils::deserialize_string_to_u64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticlesSaveRequest {
    //用户ID
    pub user_id: u64,
    //文章分类ID
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub category_id: Option<u64>,
    //所属所有的父级至子级分类
    pub category_ids: Option<String>,
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
}

impl From<ArticlesSaveRequest> for Articles {
    fn from(arg: ArticlesSaveRequest) -> Self {
        Self {
            id: 0,
            short_url: None,
            category_id: arg.category_id,
            category_ids: arg.category_ids,
            user_id: arg.user_id,
            user_type: 0,
            title: arg.title,
            short_title: arg.short_title,
            title_image: arg.title_image,
            author: arg.author,
            original_link: arg.original_link,
            description: arg.description,
            content: arg.content,
            count_comment: 0.into(),
            count_view: 0.into(),
            count_love: 0.into(),
            count_digg: 0.into(),
            count_burys: 0.into(),
            count_follow: 0.into(),
            weight: Option::from(Decimal::from_str("0").unwrap_or_default()),
            istop: 0,
            isclose: 0,
            iscomment: 0,
            iscommentshow: 0,
            isposts: 0,
            isaudit: 0,
            deleted: 0,
            isrecommend: 0,
            status: 0,
            create_time: Option::from(DateTime::now()),
            update_time: Option::from(DateTime::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticlesUpdateRequest {
    ///用户的 UID
    pub id: u64,
    //用户ID
    pub user_id: u64,
    //文章分类ID
    pub category_id: Option<u64>,
    //所属所有的父级至子级分类
    pub category_ids: Option<String>,
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
    //0未审核，1审核，2未通过
    pub status: i8,
}

impl From<ArticlesUpdateRequest> for Articles {
    fn from(arg: ArticlesUpdateRequest) -> Self {
        Self {
            id: arg.id,
            short_url: None,
            category_id: arg.category_id,
            category_ids: arg.category_ids,
            user_id: arg.user_id,
            user_type: 0,
            title: arg.title,
            short_title: arg.short_title,
            title_image: arg.title_image,
            author: arg.author,
            original_link: arg.original_link,
            description: arg.description,
            content: arg.content,
            count_comment: 0.into(),
            count_view: 0.into(),
            count_love: 0.into(),
            count_digg: 0.into(),
            count_burys: 0.into(),
            count_follow: 0.into(),
            weight: Option::from(Decimal::from_str("0").unwrap_or_default()),
            istop: 0,
            isclose: 0,
            iscomment: 0,
            iscommentshow: 0,
            isposts: 0,
            isaudit: 0,
            deleted: 0,
            isrecommend: 0,
            status: 0,
            create_time: Option::from(DateTime::now()),
            update_time: Option::from(DateTime::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticlesDetailData {
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
    ///是否关闭帖子
    pub isclose: i8,
    //是否允许评论
    pub iscomment: i8,
    //是否评论后显示内容
    pub iscommentshow: i8,
    //是否精华帖子
    pub isposts: i8,
    //1为推荐
    pub isrecommend: i8,
    //更新时间
    pub update_time: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticlesPageRequest {
    //按文章分类ID查询
    pub category_id: Option<String>,
    //按用户ID查询
    pub user_id: Option<String>,
    // 审核状态
    pub status: Option<String>,
    #[serde(rename = "pageNum")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesPageData {
    //文章ID
    pub id: u64,
    //文章分类ID
    pub category_id: Option<u64>,
    //所属所有的父级至子级分类
    pub category_ids: Option<String>,
    //用户ID
    pub user_id: u64,
    //文章标题
    pub title: Option<String>,
    //文章主图
    pub title_image: Option<String>,
    //文章展示数
    pub count_view: Option<u64>,
    //是否精华帖子
    pub isposts: i8,
    //1为推荐
    pub isrecommend: i8,
    //创建时间
    pub create_time: String,
    //0未审核，1审核，2未通过
    pub status: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticlesListRequest {
    //按文章分类ID查询
    pub category_id: Option<u64>,
    //按用户ID查询
    pub user_id: Option<u64>,
    // 审核状态
    pub status: Option<i8>,
    pub page_no: u64,
    pub page_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticlesListData {
    //文章ID
    pub id: u64,
    //短网址
    pub short_url: Option<String>,
    //文章分类ID
    pub category_id: Option<u64>,
    //所属所有的父级至子级分类
    pub category_ids: Option<String>,
    //用户信息
    pub user: ArticleUser,
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
    //文章展示数
    pub count_view: Option<u64>,
    //喜欢数
    pub count_love: Option<u64>,
    //支持数
    pub count_digg: Option<u64>,
    //反对次数
    pub count_burys: Option<u64>,
    //关注话题统计数
    pub count_follow: Option<u64>,
    ///是否关闭帖子
    pub isclose: i8,
    //是否允许评论
    pub iscomment: i8,
    //是否评论后显示内容
    pub iscommentshow: i8,
    //是否精华帖子
    pub isposts: i8,
    //1为推荐
    pub isrecommend: i8,
    //更新时间
    pub update_time: Option<String>,
    //创建时间
    pub create_time: Option<String>,
}

impl From<Articles> for ArticlesListData {
    fn from(arg: Articles) -> Self {
        Self {
            id: arg.id,
            short_url: arg.short_url,
            category_id: arg.category_id,
            category_ids: arg.category_ids,
            user:  ArticleUser::default(),
            title: arg.title,
            short_title: arg.short_title,
            title_image: arg.title_image,
            author: arg.author,
            original_link: arg.original_link,
            description: arg.description,
            content: arg.content,
            count_comment: arg.count_comment,
            count_view: arg.count_view,
            count_love: arg.count_love,
            count_digg: arg.count_digg,
            count_burys: arg.count_burys,
            count_follow: arg.count_follow,
            isclose: arg.isclose,
            iscomment: arg.iscomment,
            iscommentshow: arg.iscommentshow,
            isposts: arg.isposts,
            isrecommend: arg.isrecommend,
            update_time: arg.update_time.map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
            create_time: arg.create_time.map(|t| t.format("YYYY-MM-DD hh:mm:ss")),
        }
    }
}


#[derive(Deserialize)]
pub struct Info {
    pub category: String,
    pub page: Option<u64>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArticleUser {
    ///用户的 UID
    pub id: u64,
    ///短网址
    pub short_url: Option<String>,
    ///用户名
    pub user_name: String,
    ///昵称
    pub nick_name: Option<String>,
    ///EMAIL
    pub email: Option<String>,
    ///用户手机
    pub mobile: Option<String>,
    ///头像文件
    pub avatar_file: Option<String>,
}


impl From<User> for ArticleUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            short_url: user.short_url,
            user_name: user.user_name,
            nick_name: user.nick_name,
            email: user.email,
            mobile: user.mobile,
            avatar_file: user.avatar_file,
        }
    }
}