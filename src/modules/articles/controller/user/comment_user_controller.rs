/*#[post("/user/add_comment")]
pub async fn post_comment(user: Option<Identity>,item: web::Json<Comment>) -> HttpResponse {

    if let Some(user) = user {
        let mut comment = item.0;
        let users: UserLoginSession = serde_json::from_str(&user.id().unwrap()).unwrap();
        comment.user_id = users.id.clone();
        let result = article_service::save_comment(comment).await;
    }
}*/