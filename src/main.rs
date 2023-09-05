use std::{env, io};
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let port = env::args().nth(1).unwrap_or("8000".to_string());
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting server at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
        .bind(("127.0.0.1", port.parse().unwrap()))?
        .run()
        .await
}
