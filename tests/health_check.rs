use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use sqlx::types::Uuid;
use zero2prod::configuration::{DatabaseSettings, get_configuration};
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), default_filter_level, std::io::stdout);
        init_subscriber(subscriber);

    } else {
        let subscriber = get_subscriber("test".into(), default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to load config");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(&configuration.database).await;

    let server = zero2prod::run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: pool
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(
        // &config.connection_string_without_db().expose_secret()
        &config.without_db()
        )
        .await
        .expect("Failed to connect to Postgres");
    connection.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str()).await.expect("Failed to create database");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let url = &format!("{}/health_check", app.address);
    let path = url.as_str();
    println!("address {url}");
    let response = client
        .get(path)
        .send()
        .await
        .expect("Failed to execute request!!!");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=tay%20tayo&email=shadrachtemitayo%40gmail.com";
    let response = client.post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptioin");

    assert_eq!(saved.email, "shadrachtemitayo@gmail.com");
    assert_eq!(saved.name, "tay tayo");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client.post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customized error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

}