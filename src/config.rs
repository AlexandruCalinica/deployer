use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub local_volumes: Vec<String>,
}

pub fn read_from(data: &str) -> Result<Config> {
    let cfg: Config = serde_json::from_str(data)?;
    Ok(cfg)
}
