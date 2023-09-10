use actix_web::{web, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use sqlx::{PgPool};
use sqlx::types::{chrono, uuid};
use uuid::Uuid;
use self::chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct SubscribeParams {
    pub name: String,
    pub email: String,
}
pub async fn subscribe(
    form: web::Form<SubscribeParams>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
