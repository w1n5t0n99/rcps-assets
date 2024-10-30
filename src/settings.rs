use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
    pub session_secret: String,
}

impl ApplicationConfig {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GoogleConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub auth_url: String,
    pub token_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LocalStorageConfig {
    pub serve_path: String,
    pub route_path: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub application: ApplicationConfig,
    pub google: GoogleConfig,
    pub local_storage: LocalStorageConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("./configuration/base"))
            .add_source(File::with_name(&format!("./configuration/{}", run_mode)).required(false),
            )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()?;
        
        s.try_deserialize()
    }
}

