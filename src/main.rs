use zero2prod::configuration::get_configuration;
use zero2prod::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use sqlx::{PgPool};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
        let subscriber = get_subscriber("zerotoprod".into(), "info".into(), std::io::stdout);
        init_subscriber(subscriber);
        let settings = get_configuration().expect("Failed to read configuration");
        let address = format!("{}:{}", settings.application.host, settings.application.port);
        tracing::info!("application address {}: {address}", settings.application.host);
        let connection_string = settings.database.connection_string();
        let connection_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy(&connection_string.expose_secret()).expect("Failed to get connection");
        let listener = TcpListener::bind(address).expect("Failed to bind port");
        // tracing::info!("listener {}", listener.local_add);
        run(listener, connection_pool)?.await
}
