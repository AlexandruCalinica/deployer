use structopt::StructOpt;

mod cli;
mod command;
mod config;

use crate::cli::Cli;
use crate::command::{cleanup, run, run_local};
use crate::config::{load_config, LocalVolume};

fn main() {
    let cfg = load_config();

    match Cli::from_args() {
        Cli::Run { all, jobs, local } => {
            if all && local && jobs.len() == 0 {
                println!("run all jobs local");
                for vol in cfg.local_volumes.iter() {
                    println!("run local {:?}", vol.name);
                    run_local(&vol.path, &vol.name, &vol.command, &vol.port_map)
                }
            } else if local && jobs.len() > 0 {
                let filtered: Vec<LocalVolume> = cfg
                    .local_volumes
                    .into_iter()
                    .filter(|vol| jobs.iter().any(|j| j == &vol.name))
                    .collect();

                for vol in filtered.iter() {
                    println!("run local {}", vol.name);
                    run_local(&vol.path, &vol.name, &vol.command, &vol.port_map)
                }
            } else if all && !local && jobs.len() == 0 {
                println!("run all jobs from docker image");
                run("docker-compose", vec!["up"]);
            }
        }
        Cli::Clean { all, names } => {
            if all {
                for vol in cfg.local_volumes.iter() {
                    cleanup(&vol.name);
                    println!("cleaned {}", vol.name);
                }
            } else if names.len() > 0 {
                let filtered: Vec<LocalVolume> = cfg
                    .local_volumes
                    .into_iter()
                    .filter(|vol| names.iter().any(|n| n == &vol.name))
                    .collect();

                for vol in filtered.iter() {
                    println!("cleaned {}", vol.name);
                    cleanup(&vol.name);
                }
            }
        }
    }
}
