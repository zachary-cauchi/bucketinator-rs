use std::env;

use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BucketinatorConfiguration {
    pub version: String,
    pub db_file_path: String,
}

impl BucketinatorConfiguration {
    pub fn get_config() -> Result<Self, ConfigError> {
        let conf_path = env::var("BUCKETINATOR_CONF").unwrap_or("./bucketinator.toml".to_string());

        let settings_config = Config::builder()
            .add_source(config::File::with_name(conf_path.as_str()))
            .add_source(config::Environment::with_prefix("BUCKETINATOR"))
            .build()?;

        settings_config.try_deserialize()
    }
}
