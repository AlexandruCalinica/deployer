use dialoguer::{theme::ColorfulTheme, Select};
use structopt::StructOpt;

mod cli;
mod command;
mod config;
mod dialog;
mod util;

use crate::cli::{clean_all, clean_some, compose_up, run_all, run_all_local, run_some_local, Cli};
use crate::config::load_config;
use crate::dialog::{start_from_image, start_from_local};
use crate::util::logger;

fn main() {
    let cfg = load_config();

    match Cli::from_args() {
        Cli::Run { all, jobs, local } => {
            if all && local && jobs.len() == 0 {
                run_all_local(cfg);
            } else if local && jobs.len() > 0 {
                run_some_local(cfg, jobs);
            } else if all && !local && jobs.len() == 0 {
                run_all();
            }
        }
        Cli::Clean { all, names } => {
            if all {
                clean_all(cfg);
            } else if names.len() > 0 {
                clean_some(cfg, names);
            }
        }
        Cli::Start {} => {
            logger("Start a single or multiple selection of docker containers.");
            let entry_opt = &["image", "local"];
            let entry_sel = Select::with_theme(&ColorfulTheme::default())
                .items(entry_opt)
                .default(0)
                .interact()
                .unwrap();

            match entry_sel {
                0 => start_from_image(&cfg),
                1 => start_from_local(&cfg),
                _ => println!("Option not available"),
            }
        }
        Cli::Up {} => {
            logger("Start docker-compose in dev mode.");
            compose_up();
        }
    }
}
