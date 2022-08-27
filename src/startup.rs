use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_lab::middleware::from_fn;
use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{health_check, home};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host,
            configuration.application.port,
        );

        let listener = TcpListener::bind(&address)?;
        // In production (port 0) the OS will assign an open port.
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            configuration.application.base_url,
            configuration.application.hmac_secret,
        )
        .await?;

        Ok(Self{ port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

async fn run (
    listener: TcpListener,
    base_url: String,
    hmac_secret: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    //let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/health_check", web::get().to(health_check))
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();

    Ok(server)
}