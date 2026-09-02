use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub ip: String,
    pub port: u16
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub server: ServerSettings,
}

impl ServerSettings {
    pub fn to_addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
