pub use ::config::{Config, ConfigError, Environment};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ConfigApp {
    pub server_addr:    String,
    pub pg:             deadpool_postgres::Config,
}
impl ConfigApp {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        
        cfg.merge(Environment::new())?;
        cfg.try_into()
    }
}

pub mod error;