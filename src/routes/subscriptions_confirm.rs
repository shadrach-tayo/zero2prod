use crate::domain::SubscriberToken;
use crate::utils::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use std::fmt::Formatter;
use uuid::Uuid;

// pub struct

#[derive(thiserror::Error)]
pub enum ConfirmError {
    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ConfirmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ConfirmError {
    fn status_code(&self) -> StatusCode {
        match self {
            ConfirmError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ConfirmError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

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
pub async fn confirm(
    params: web::Query<Params>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ConfirmError> {
    let subscription_token: SubscriberToken = params
        .subscription_token
        .clone()
        .try_into()
        .map_err(ConfirmError::ValidationError)?;

    let id = get_subscriber_id_from_token(&pool, subscription_token.as_ref())
        .await
        .unwrap()
        .context("Failed to retrieve subscriber")?;

    confirm_subscriber(&pool, id)
        .await
        .context("Failed to Confirm subscriber")?;
    expire_subscription_token(&pool, subscription_token.as_ref())
        .await
        .context("Invalid subscription token")?;

    Ok(HttpResponse::Ok().finish())
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
    .await?;
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
