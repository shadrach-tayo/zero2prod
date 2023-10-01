// use std::net::TcpListener;
// use actix_web::{middleware, web, App, HttpServer};
// use crate::email_client::EmailClient;
// use actix_web::dev::Server;
// use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;

// async fn index(form: web::Form<SubscribeParams>) -> String {
//     format!("Welcome {}!", form.name)
// }
// pub fn run(
//     listener: TcpListener,
//     db_pool: PgPool,
//     email_client: EmailClient,
// ) -> Result<Server, std::io::Error> {
//     startup::run(listener, db_pool, email_client, "".to_string())
// }
