use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub web_config: WebConfig,
    pub postgres_config: PostgresConfig,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    domain: String,
    db_name: String,
}

impl PostgresConfig {
    pub fn make_address(&self) -> String {
        format!("{}/{}", self.domain, self.db_name)
    }
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config: Config =
            toml::from_str(include_str!("configs.toml")).map_err(|err| err.to_string())?;
        Ok(config)
    }
}
