use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub local_volumes: Vec<LocalVolume>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalVolume {
    pub path: String,
    pub name: String,
    pub command: String,
    pub port_map: String,
}

pub fn read_from(data: &str) -> Result<Config> {
    let cfg: Config = serde_json::from_str(data)?;
    Ok(cfg)
}

pub fn load_config() -> Config {
    let content = fs::read_to_string("./deplo/deplo.json")
        .expect("could not read deplo.json configuration file");
    let config: Config;

    match read_from(&content) {
        Ok(res) => {
            config = res;
        }
        Err(err) => {
            println!("{}", err);
            config = Config {
                local_volumes: vec![],
            }
        }
    };
    config
}
