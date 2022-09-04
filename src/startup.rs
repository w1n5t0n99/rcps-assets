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
use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{health_check, get_image, home, new_asset_form, new_asset, asset_items_form, get_asset, edit_asset_form, edit_asset};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host,
            configuration.application.port,
        );

        let listener = TcpListener::bind(&address)?;
        // In production (port 0) the OS will assign an open port.
        let port = listener.local_addr().unwrap().port();

        println!("Server runing on {}:{}", configuration.application.host, port);

        let server = run(
            listener,
            connection_pool,
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

pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

async fn run (
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let db_pool = Data::new(db_pool);

    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(message_framework.clone())
            .route("/", web::get().to(home))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}.png", web::get().to(get_image))
            .route("/asset_items/new", web::get().to(new_asset_form))
            .route("/asset_items/new", web::post().to(new_asset))
            .route("/asset_items/{id}", web::get().to(get_asset))
            .route("/asset_items/{id}/edit", web::get().to(edit_asset_form))
            .route("/asset_items/{id}/edit", web::post().to(edit_asset))
            .route("/asset_items", web::get().to(asset_items_form))
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}