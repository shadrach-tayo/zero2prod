use actix_web::{web, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use sqlx::{PgPool};
use sqlx::types::{chrono, uuid};
use uuid::Uuid;
use self::chrono::Utc;
use tracing::{Instrument};

#[derive(Serialize, Deserialize)]
pub struct SubscribeParams {
    pub name: String,
    pub email: String,
}
pub async fn subscribe(
    form: web::Form<SubscribeParams>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();

    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    // Using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't do this at home.
    // See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );

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
        .instrument(query_span)
    .await {
        Ok(_) => {
            // tracing::info!("request_id {} - Saving new subscriber details in the database", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            // println!("Failed to execute query: {}", e);
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
