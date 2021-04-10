use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    address: String,
    port: Option<u32>,
    workers: Option<u32>,
    environment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    adapter: String,
    db_name: String,
    pool: Option<u32>,
}
