use serde::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub database_url: String,
}
