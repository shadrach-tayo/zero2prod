use std::net::TcpListener;
use sqlx::{PgConnection, Connection, PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
        let settings = get_configuration().expect("Failed to read configuration");
        let address = format!("127.0.0.1:{}", settings.application_port);
        let connection_string = settings.database.connection_string();
        let connection_pool = PgPool::connect(&connection_string).await.expect("Failed to get connection");
        let listener = TcpListener::bind(address).expect("Failed to bind port");
        run(listener, connection_pool)?.await
}
