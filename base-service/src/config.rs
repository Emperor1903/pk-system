use serde::Deserialize;
use config::ConfigError;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub domain: String,
}

#[derive(Deserialize)]
pub struct MongodbConfig {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub mongodb: MongodbConfig,
    pub imgbb_api: String
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
