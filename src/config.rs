use config::ConfigError;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .set_default("server.host", "localhost")?
            .set_default("server.port", "8080")?
            .add_source(config::Environment::with_prefix("ACTIX").separator("_"))
            .build()
            .unwrap();
        cfg.try_deserialize()
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[test]
fn server_config_to_string() {
    let server = ServerConfig {
        host: String::from("localhost"),
        port: 24u16,
    };

    assert_eq!("localhost:24", server.to_string());
}
