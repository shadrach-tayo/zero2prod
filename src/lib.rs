use std::net::TcpListener;
use actix_web::{middleware, web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::{PgConnection, PgPool};

use crate::routes::{health_check, subscribe, SubscribeParams};

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

async fn index(form: web::Form<SubscribeParams>) -> String {
    format!("Welcome {}!", form.name)
}
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();
    tracing::info!("starting server at http://localhost:{}", port);
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}
