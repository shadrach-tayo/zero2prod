use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
        let settings = get_configuration().expect("Failed to read configuration");
        let address = format!("127.0.0.1:{}", settings.application_port);
        let listener = TcpListener::bind(address).expect("Failed to bind port");
        run(listener)?.await
}
