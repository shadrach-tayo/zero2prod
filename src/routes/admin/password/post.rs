use crate::authentication::{validate_credentials, AuthError, Credentials, UserId};
use crate::domain::{ChangePasswordParam, Password};
use crate::routes::get_username;
use crate::utils::{e500, see_other};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use serde_derive::Deserialize;
use sqlx::PgPool;
// use std::str::FromStr;

#[derive(Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

impl TryFrom<FormData> for ChangePasswordParam {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        Ok(Self {
            current_password: Password::parse(value.current_password.expose_secret().to_owned())?,
            new_password: Password::parse(value.new_password.expose_secret().to_owned())?,
            new_password_check: Password::parse(
                value.new_password_check.expose_secret().to_owned(),
            )?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangePasswordError {
    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for ChangePasswordError {
    fn status_code(&self) -> StatusCode {
        match self {
            ChangePasswordError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ChangePasswordError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ChangePasswordError::ValidationError(err) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(&serde_json::json!({"error": err}))
            }
            ChangePasswordError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::BAD_REQUEST).finish()
            }
        }
    }
}

#[tracing::instrument(name = "Change password", skip(form, user_id, pool))]
pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let change_password_param: ChangePasswordParam = form
        .0
        .try_into()
        .map_err(ChangePasswordError::ValidationError)?;

    if change_password_param.new_password_check.as_ref()
        != change_password_param.new_password.as_ref()
    {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }

    let username = get_username(*user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: Secret::new(change_password_param.current_password.as_ref().to_owned()),
    };

    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }
    crate::authentication::change_password(
        *user_id,
        Secret::new(change_password_param.new_password.as_ref().to_string()),
        &pool,
    )
    .await
    .map_err(e500)?;
    FlashMessage::info("Your password has been changed.").send();
    Ok(see_other("/admin/dashboard"))
}
