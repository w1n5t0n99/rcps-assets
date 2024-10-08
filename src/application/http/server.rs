use anyhow::{Context, Result};
use axum::{extract::State, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Json};
use axum_extra::extract::cookie::SameSite;
use axum_login::{tower_sessions::{Expiry, MemoryStore, SessionManagerLayer}, AuthManagerLayerBuilder};
use tower_sessions::cookie::Key;
use time::Duration;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::instrument;
use axum_messages::{Messages, MessagesManagerLayer};

use crate::{application::{identityaccess::identity_application_service::IdentityApplicationService, state::AppState}, domain::identityaccess::model::user_repository::UserRepository, infastructure::services::postgres_user_repository::PostgresUserRepository, settings::ApplicationConfig};
use super::{handlers::{settings, auth, oauth}, utils};


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
    pub async fn new<U: UserRepository>(config: &ApplicationConfig, app_state: AppState<U>) -> Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http();
        let compression_layer = tower_http::compression::CompressionLayer::new().br(true);

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
            .merge(settings::router())
            .merge(oauth::router())
            .merge(auth::router())
            .nest_service("/static", ServeDir::new("static").precompressed_gzip())
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
