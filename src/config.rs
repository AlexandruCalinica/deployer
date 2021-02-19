use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub local_volumes: Vec<LocalVolume>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalVolume {
    pub path: String,
    pub name: String,
}

pub fn read_from(data: &str) -> Result<Config> {
    let cfg: Config = serde_json::from_str(data)?;
    Ok(cfg)
}
