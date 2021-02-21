use crate::command::run_local;
use crate::config::Config;
use crate::util::map_volumes_names;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

pub fn start_from_image(cfg: &Config) {
    let loc_vols = map_volumes_names(cfg);

    let mut defaults: Vec<bool> = vec![];
    for _ in loc_vols.iter() {
        defaults.push(true);
    }

    let selections: Vec<usize> = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&loc_vols)
        .defaults(&defaults)
        .interact()
        .unwrap();

    println!("{:?}", selections)
}

pub fn start_from_local(cfg: &Config) {
    let loc_vols_names = map_volumes_names(cfg);
    let loc_vols = &cfg.local_volumes;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&loc_vols_names)
        .default(0)
        .interact()
        .unwrap();

    let vol = &loc_vols[selection];
    run_local(&vol.path, &vol.name, &vol.command, &vol.port_map);
}
