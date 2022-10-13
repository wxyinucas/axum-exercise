use serde::{Deserialize, Serialize};

fn main() {
    let config: Config = toml::from_str(include_str!("../../configs/test_config.toml")).unwrap();
    println!("{:?}", config)
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
// todo 注意字段间的对应关系
struct Config {
    web_config: WebConfig,
    redis_config: RedisConfig,
    postgres_config: PostgresConfig,
}

#[derive(Deserialize, Debug, Serialize)]
struct WebConfig {
    addr: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct RedisConfig {
    addr: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct PostgresConfig {
    domain: String,
    db_name: String,
}

#[allow(dead_code)]
impl PostgresConfig {
    fn make_address(&self) -> String {
        format!("{}/{}", self.domain, self.db_name)
    }
}
