use std::{fs, path};

use config::LocalVolume;
use structopt::StructOpt;

mod cli;
mod command;
mod config;

use crate::cli::Cli;
use crate::command::run;
use crate::config::{read_from, Config};

fn main() {
    let cfg = load_config();

    match Cli::from_args() {
        Cli::Run { all, jobs, local } => {
            if all && local && jobs.len() == 0 {
                println!("run all jobs local");
                for vol in cfg.local_volumes.iter() {
                    println!("run local {:?}", vol.name);
                    println!();
                    run_local(&vol.path, &vol.name)
                }
            } else if local && jobs.len() > 0 {
                let filtered: Vec<LocalVolume> = cfg
                    .local_volumes
                    .into_iter()
                    .filter(|vol| jobs.iter().any(|j| j == &vol.name))
                    .collect();

                for vol in filtered.iter() {
                    println!("run local {}", vol.name);
                    println!();
                    run_local(&vol.path, &vol.name)
                }
            } else if all && !local && jobs.len() == 0 {
                println!("run all jobs from docker image");
                println!();
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
                    println!();
                    cleanup(&vol.name);
                }
            }
        }
    }
}

pub fn run_local(target: &str, name: &str) {
    cleanup(name);

    let target_path = path::PathBuf::from(target);
    let absolute_target_path = fs::canonicalize(&target_path).unwrap();

    let cmd = format!(
        r#"run -d --name {} -v {:#?}:/app node:alpine"#,
        name, absolute_target_path
    )
    .replace("\"", "");

    let splitted: Vec<&str> = cmd.split(' ').collect();

    run("docker", splitted);
}

pub fn cleanup(name: &str) {
    run("docker", vec!["rm", name]);
}

pub fn load_config() -> Config {
    let content = fs::read_to_string("deployer.json").expect("could not read file");
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
