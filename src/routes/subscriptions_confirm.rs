use crate::domain::SubscriberToken;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct Params {
    subscription_token: String,
}

impl TryFrom<String> for SubscriberToken {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        SubscriberToken::parse(value)
    }
}
#[tracing::instrument(name = "Confirm a pending subscriber", skip(params, pool))]
pub async fn confirm(params: web::Query<Params>, pool: web::Data<PgPool>) -> HttpResponse {
    let subscription_token: SubscriberToken = match params.subscription_token.clone().try_into() {
        Ok(subscription_token) => subscription_token,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let id = match get_subscriber_id_from_token(&pool, subscription_token.as_ref()).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match id {
        None => HttpResponse::Unauthorized().finish(),
        Some(subscriber_id) => {
            if confirm_subscriber(&pool, subscriber_id).await.is_err() {}
            if expire_subscription_token(&pool, subscription_token.as_ref())
                .await
                .is_err()
            {}
            HttpResponse::Ok().finish()
        }
    }
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(pool, subscription_token))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        subscription_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(pool, subscriber_id))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Mark subscription_token as expired",
    skip(pool, subscription_token)
)]
pub async fn expire_subscription_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscription_tokens SET expired = true WHERE subscription_token = $1"#,
        subscription_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
