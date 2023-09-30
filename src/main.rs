use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zerotoprod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = get_configuration().expect("Failed to read configuration");
    tracing::info!(
        "application address {}: {}",
        settings.application.host,
        settings.application.port
    );
    let application = Application::build(settings).await?;
    application.run_until_stopped().await?;
    Ok(())
}
