mod telemetry;

mod domain;
mod settings;
mod infastructure;
mod application;

use anyhow::{Context, Ok};
use application::{content::content_application_service::ContentApplicationService, crud::crud_application_service::CrudApplicationService, http::server::AppHttpServer, identityaccess::identity_application_service::IdentityApplicationService, state::AppState};
use domain::filesystem::persistence_service;
use infastructure::services::{google_oauth_service::GoogleOauthService, local_persistence_service::LocalPersistenceService, postgres_attachment_repository::PostgresAttachmentRepository, postgres_crud_repository::PostgresCrudRepository, postgres_user_repository::PostgresUserRepository};
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
    let attachment_repo = PostgresAttachmentRepository::new(&config.database).context("failed to init attachment repository")?;
    let crud_repo = PostgresCrudRepository::new(&config.database).context("failed to init attachment repository")?;
    let persistence = LocalPersistenceService::new(&config.local_storage.route_path, &config.local_storage.serve_path).context("failed to init persistence repository")?;

    let content_service = ContentApplicationService::new(attachment_repo, persistence);
    let identity_serivce = IdentityApplicationService::new(user_repo, google_oauth);
    let crud_service = CrudApplicationService::new(crud_repo);

    //init server
    let app_state = AppState::new(identity_serivce, crud_service, content_service);
    let app_server = AppHttpServer::new(&config.application, app_state).await?;

    // run tasks
    app_server.run_until_stopped().await?;

    Ok(())
}