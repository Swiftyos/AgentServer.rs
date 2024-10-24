use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

/// Configuration struct for the REST service
#[derive(Debug, Deserialize)]
pub struct RestConfig {
    /// URL of the database
    ///
    /// This field specifies the connection string for the database used by the REST service.
    /// It typically includes the database type, host, port, and database name.
    pub database_url: String,

    /// Schema name in the database
    ///
    /// This field defines the specific schema within the database where the service's tables are located.
    /// It helps in organizing and separating data for different applications or modules.
    pub database_schema: String,

    /// Host address for the server
    ///
    /// This field specifies the IP address or domain name on which the REST service will listen for incoming requests.
    /// It can be set to a specific address or "0.0.0.0" to listen on all available network interfaces.
    pub server_host: String,

    /// Port number for the server
    ///
    /// This field defines the TCP port number on which the REST service will listen for incoming connections.
    /// It should be an available port on the host system, typically in the range of 1024-65535.
    pub server_port: u16,
}

impl RestConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("../../config/rest_config.toml").required(false))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("../../config/rest_config.{}.toml", run_mode))
                    .required(false),
            )
            // Add in settings from the environment (with a prefix of AGPT_REST)
            // Eg.. `AGPT_REST_SERVER_PORT=5001 would set `RestConfig.server_port`
            .add_source(Environment::with_prefix("AGPT_REST"))
            .build()?;

        s.try_deserialize()
    }
}
