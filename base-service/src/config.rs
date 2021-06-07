use serde::Deserialize;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

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
pub struct EmailConfig {
    pub server: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub mongodb: MongodbConfig,
    pub email: EmailConfig,
}

impl Config {
    pub fn from_env() -> Self {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new()).unwrap();
        cfg.try_into().unwrap()
    }
}
