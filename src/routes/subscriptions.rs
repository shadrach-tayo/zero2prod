use actix_web::{HttpResponse, web};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubscribeParams {
    pub name: String,
    pub email: String
}
pub async fn subscribe(_form: web::Form<SubscribeParams>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
