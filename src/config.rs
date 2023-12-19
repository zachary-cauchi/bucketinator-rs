use std::env;

use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RustodosConfiguration {
    pub version: String,
    pub db_file_path: String,
}

impl RustodosConfiguration {
    pub fn get_config() -> Result<Self, ConfigError> {
        let conf_path = env::var("RUSTODOS_CONF").unwrap_or("./rustodos.toml".to_string());

        let settings_config = Config::builder()
            .add_source(config::File::with_name(conf_path.as_str()))
            .add_source(config::Environment::with_prefix("RUSTODOS"))
            .build()?;

        settings_config.try_deserialize()
    }
}
