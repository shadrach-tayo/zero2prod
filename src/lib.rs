use std::net::TcpListener;
// use actix_web::{middleware, web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::{PgPool};

use crate::routes::{health_check, subscribe};

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

// async fn index(form: web::Form<SubscribeParams>) -> String {
//     format!("Welcome {}!", form.name)
// }
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    startup::run(listener, db_pool)
}
