//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!


/*#[post("/user/add_comment")]
pub async fn post_comment(user: Option<Identity>,item: web::Json<Comment>) -> HttpResponse {

    if let Some(user) = user {
        let mut comment = item.0;
        let users: UserLoginSession = serde_json::from_str(&user.id().unwrap()).unwrap();
        comment.user_id = users.id.clone();
        let result = article_service::save_comment(comment).await;
    }
}*/