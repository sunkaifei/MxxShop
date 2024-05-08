use crate::modules::upload::entity::attach_entity::ImageForm;
use crate::modules::upload::service::attach_service;
use actix_multipart::form::MultipartForm;
use actix_web::HttpResponse;

// 上传图片
pub async fn upload_totem_image(form: MultipartForm<ImageForm>, ) -> HttpResponse {
    let form = form.into_inner();

    return attach_service::upload_totem_image("totem".to_string(), form).await;
}

