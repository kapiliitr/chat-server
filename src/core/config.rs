use envconfig::Envconfig;
use models::ChatErrorKind::ConfigInitializationFailed;
use models::{ChatError, ChatResult};

#[derive(Envconfig, Debug)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_HOSTNAME", default = "127.0.0.1")]
    pub hostname: String,
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub port: String,
    #[envconfig(from = "MAX_NUM_CONNECTIONS", default = "10")]
    pub max_num_connections: usize,
}

impl ServerConfig {
    pub fn new() -> ChatResult<ServerConfig> {
        ServerConfig::init().map_err(|e| ChatError::new_with_error(ConfigInitializationFailed, e))
    }
}
