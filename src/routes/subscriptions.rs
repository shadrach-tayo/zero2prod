use actix_web::{web, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use sqlx::{PgPool};
use sqlx::types::{chrono, uuid};
use uuid::Uuid;
// use tracing::{Instrument};

#[derive(Serialize, Deserialize)]
pub struct SubscribeParams {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(
    form: web::Form<SubscribeParams>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }

}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool),
)]
pub async fn insert_subscriber(
    form: &SubscribeParams,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())

}
