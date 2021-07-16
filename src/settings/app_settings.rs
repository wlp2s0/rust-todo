use std::env;

use config::{Config, Environment, File};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static APP_SETTINGS: Lazy<Settings> = Lazy::new(Settings::init_config);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerSettings {
    /// Client's uri
    pub clienturi: String,

    /// Server's port
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// HTTP server and app configuration
    pub server: ServerSettings,
}

impl Settings {
    fn init_config() -> Self {
        // Start config
        let mut s = Config::default();

        // Create a path
        let mut config_file_path = env::current_dir().expect("Cannot get current path");

        // Get current RUN_MODE, should be: development/production
        let current_env = env::var("RUN_MODE").unwrap_or_else(|_| String::from("development"));

        // Get current LOCAL, should be: true when running locally
        let local = env::var("LOCAL").unwrap_or_default() == "true";

        // From current path add /environments
        config_file_path.push("environments");

        // ex. development/production
        let mut filename = current_env;
        // Add local
        if local {
            filename.push_str(".local");
        }
        // Add extension
        filename.push_str(".yaml");

        // Add RUN_MODE{.local}.yaml
        config_file_path.push(filename);

        // Add in the current environment file
        // Default to 'development' env
        s.merge(File::from(config_file_path).required(false))
            .expect("Could not read file");

        // Add in settings from the environment
        // ex. APP_DEBUG=1 sets debug key, APP_DATABASE_URL sets database.url key
        s.merge(Environment::new().prefix("APP").separator("_"))
            .expect("Cannot get env");

        // Deserialize configuration
        let r: Settings = s.try_into().expect("Configuration error");

        r
    }
}
