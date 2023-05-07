use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool, Row};
use uuid::Uuid;
use first::configuration::{DatabaseSettings, get_configuration};

pub struct TestApp {
    address: String,
    connection_pool: PgPool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let mut config = get_configuration().expect("Can't read config");
    config.database.db_name = Uuid::new_v4().to_string();
    let db_pool = create_db(&config.database).await;

    let server = first::startup::
        run(listener, db_pool.clone()).expect("Server failed to start");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        connection_pool: db_pool
    }
}

pub async fn create_db(config: &DatabaseSettings) -> PgPool {
    let mut con = PgConnection::connect(&config.to_string_no_db_name())
        .await
        .expect("Can't connect to db");
    con.execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.to_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Can't migrate database");

    connection_pool
}

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=test&email=test%40ua.com";
    let response = client.post(format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Not valid post request");

    assert_eq!(200, response.status().as_u16());

    let result = sqlx::query("SELECT email, name FROM subscriptions WHERE name='test'")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Not valid query!");

    let name: &str = result.try_get("name").unwrap();
    let email: &str = result.try_get("email").unwrap();
    assert_eq!(name, "test");
    assert_eq!(email, "test@ua.com");
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
        let response = client
            .post(&format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
