use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json};
use axum_extra::extract::cookie::SameSite;
use axum_login::{tower_sessions::{Expiry, SessionManagerLayer}, AuthManagerLayerBuilder};
use tower_sessions::cookie::Key;
use time::Duration;
use tokio::net::TcpListener;
use tower_http::{compression::{predicate::{NotForContentType, SizeAbove}, Predicate}, services::ServeDir};
use tracing::instrument;
use axum_messages::MessagesManagerLayer;

use crate::{application::state::AppState, settings::ApplicationConfig};
use super::handlers::{account, asset_items, asset_types, auth, oauth};


#[instrument]
async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "How to Implement Google OAuth2 in Rust";
    (StatusCode::OK, Json(serde_json::json!({"status": "success", "message": MESSAGE})))
}

pub struct AppHttpServer {
    port: u16,
    router: axum::Router,
    listener: TcpListener,
}

impl AppHttpServer {
    pub async fn new(config: &ApplicationConfig, app_state: AppState) -> Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http();

        let compression_predicate = SizeAbove::new(256)
            .and(NotForContentType::GRPC)
            .and(NotForContentType::SSE)
            .and(NotForContentType::IMAGES)
            .and(NotForContentType::const_new("text/csv"))
            .and(NotForContentType::const_new("application/pdf"));
        let compression_layer = tower_http::compression::CompressionLayer::new()
            .br(true)
            .compress_when(compression_predicate);

        let address = config.get_address();
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();

        let session_store = tower_sessions::MemoryStore::default(); 
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        let auth_layer = AuthManagerLayerBuilder::new(app_state.identity_service.clone(), session_layer).build();            

        let router = axum::Router::new()
            .route("/healthchecker", get(health_checker_handler))
            .merge(account::router())
            .merge(oauth::router())
            .merge(auth::router())
            .merge(asset_types::router())
            .merge(asset_items::router())
            .nest_service("/static", ServeDir::new("static").precompressed_gzip())
            .nest_service("/content", ServeDir::new("content"))
            .layer(compression_layer)
            .layer(trace_layer)
            .layer(MessagesManagerLayer)
            .layer(auth_layer)
            .with_state(app_state);
        
        Ok(Self {port, router, listener})
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;

        Ok(())
    }
}
