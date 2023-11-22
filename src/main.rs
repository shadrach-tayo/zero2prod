use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use zero2prod::configuration::get_configuration;
use zero2prod::issue_delivery_worker::run_worker_until_stopped;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zerotoprod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = get_configuration().expect("Failed to read configuration");
    tracing::info!(
        "application address {}: {}",
        settings.application.host,
        settings.application.port
    );

    let application = Application::build(settings.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(settings));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    }
    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!( error.cause_chain = ?e, error.message = %e, "{}failed", task_name )
        }
        Err(e) => {
            tracing::error!( error.cause_chain = ?e, error.message = %e, "{}' task failed to complete", task_name )
        }
    }
}
