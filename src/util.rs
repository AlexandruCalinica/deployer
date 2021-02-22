use crate::config::Config;
use console::style;

pub fn map_volumes_names(cfg: &Config) -> Vec<String> {
    cfg.local_volumes
        .iter()
        .map(|o| o.name.to_string())
        .collect()
}

pub fn logger(msg: &str) {
    println!("{}", style(msg).green());
}
