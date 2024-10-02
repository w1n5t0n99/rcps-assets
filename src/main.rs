mod telemetry;

mod domain;
mod settings;
mod infastructure;
mod application;

use anyhow::{Context, Ok};
use application::{http::server::AppHttpServer, identityaccess::identity_application_service::IdentityApplicationService, state::AppState};
use infastructure::services::{google_oauth_service::GoogleOauthService, postgres_user_repository::PostgresUserRepository};
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

    //init server
    let app_state = AppState::new(identity_serivce);
    let app_server = AppHttpServer::new(&config.application, app_state).await?;

    // run tasks
    app_server.run_until_stopped().await?;

    Ok(())
}