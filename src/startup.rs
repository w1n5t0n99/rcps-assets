use anyhow::Context;
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
use tera::Tera;
use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{health_check, home, add_asset_form, add_asset};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let tmpl = get_tera().context("Could not get tera instance.")?;

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
            connection_pool,
            tmpl,
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

fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

fn get_tera() -> Result<Tera, tera::Error> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let template_directory = base_path.join("templates")
        .into_os_string()
        .into_string()
        .expect("Could not convert path to str");

    Tera::new("templates/**/*.html")
}

pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

async fn run (
    listener: TcpListener,
    db_pool: PgPool,
    tmpl: Tera,
    base_url: String,
    hmac_secret: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let db_pool = Data::new(db_pool);
    let tmpl = Data::new(tmpl);
    //let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/health_check", web::get().to(health_check))
            .route("/assets/add", web::get().to(add_asset_form))
            .route("/assets/add", web::post().to(add_asset))
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
            .app_data(db_pool.clone())
            .app_data(tmpl.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}