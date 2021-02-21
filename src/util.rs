use crate::config::Config;

pub fn map_volumes_names(cfg: &Config) -> Vec<String> {
    cfg.local_volumes
        .iter()
        .map(|o| o.name.to_string())
        .collect()
}
