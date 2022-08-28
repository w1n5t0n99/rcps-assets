use once_cell::sync::Lazy;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use rcps_assets::telemetry::{get_subscriber, init_subscriber};
use rcps_assets::configuration::*;
use rcps_assets::startup::Application;


// Ensure that the `tracing` stack is only initialised once using `once_cell`
#[allow(dead_code)]
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApplication {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    // Used to simulate interacting with server
    pub api_client: reqwest::Client,
}

impl TestApplication {
    async fn post_form(&self, endpoint: &str, body: &impl serde::Serialize) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/{}", &self.address, endpoint))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_health_check(&self) -> reqwest::Response {
        let response = self.api_client
            .get(&format!("{}/health_check", self.address))
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }

    pub async fn post_add_asset(&self, body: &impl serde::Serialize) -> reqwest::Response {
        self.post_form("assets/add", body).await
    }
}

pub async fn spawn_test_application() -> TestApplication {
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use random port
        c.application.port = 0;
        c
    };

    let db_pool = configure_database_for_testing(&configuration.database).await;

    // Launch application as background task to not interfere with tests
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApplication {
        address: format!("http://localhost:{}", application_port),
        db_pool,
        port: application_port,
        api_client: client,
    }
}

async fn configure_database_for_testing(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
