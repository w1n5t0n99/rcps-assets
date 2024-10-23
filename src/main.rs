mod telemetry;

mod domain;
mod settings;
mod infastructure;
mod application;

use anyhow::{Context, Ok};
use application::{content::content_application_service::ContentApplicationService, http::server::AppHttpServer, identityaccess::identity_application_service::IdentityApplicationService, state::AppState};
use infastructure::services::{google_oauth_service::GoogleOauthService, local_persistence_service::LocalPersistenceService, postgres_attachment_repository::PostgresAttachmentRepository, postgres_user_repository::PostgresUserRepository};
use settings::Settings;
use telemetry::init_console_subscriber;
use tracing::Level;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //load configuration data
    let config = Settings::new()?;

    // init tracing
    let _log_gaurd = init_console_subscriber(Level::DEBUG)?;

    // init services
    let google_oauth = GoogleOauthService::new(&config.google).context("failed to init google oauth client")?;
    let user_repo = PostgresUserRepository::new(&config.database).context("failed to init user repository")?;
    let identity_serivce = IdentityApplicationService::new(user_repo, google_oauth);

    let loc_persist_service = LocalPersistenceService::new("./content")?;
    let attachment_repo = PostgresAttachmentRepository::new(&config.database).context("failed to init attachment repository")?;
    let content_service = ContentApplicationService::new(attachment_repo, loc_persist_service, "/content".to_string());

    //init server
    let app_state = AppState::new(identity_serivce, content_service);
    let app_server = AppHttpServer::new(&config.application, app_state).await?;

    // run tasks
    app_server.run_until_stopped().await?;

    Ok(())
}