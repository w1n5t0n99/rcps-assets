use once_cell::sync::Lazy;
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
    // Used to simulate interacting with server
    pub api_client: reqwest::Client,
}

impl TestApplication {
    pub async fn get_health_check(&self) -> reqwest::Response {
        let response = self.api_client
            .get(&format!("{}/health_check", self.address))
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }
}

pub async fn spawn_test_application() -> TestApplication {
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use random port
        c.application.port = 0;
        c
    };

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
        port: application_port,
        api_client: client,
    }
}
