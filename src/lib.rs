use std::net::TcpListener;
use actix_web::{middleware, web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // let port = env::args().nth(1).unwrap_or("8000".to_string());
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // log::info!("starting server at http://localhost:{}", port);
    let port = listener.local_addr().unwrap().port();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health_check", web::get().to(health_check))
    })
        .listen(listener)?
        // .bind(("127.0.0.1:8000", port.parse().unwrap()))?
        .run();
    println!("server running on {}.....", port);
    Ok(server)
}
