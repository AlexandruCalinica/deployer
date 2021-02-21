use structopt::StructOpt;

use crate::command::{cleanup, run, run_local};
use crate::config::{Config, LocalVolume};

#[structopt(
    name = "Youlo deployer",
    about = "CLI tool for orchestrating docker containers."
)]
#[derive(StructOpt, Debug)]
pub enum Cli {
    /// Run a selection of containers or all at once.
    #[structopt(name = "run")]
    Run {
        /// Array of containers ready to run.
        #[structopt(short, long)]
        jobs: Vec<String>,
        /// Run all at once.
        #[structopt(short, long)]
        all: bool,
        /// Run in development mode.
        #[structopt(short, long)]
        local: bool,
    },

    /// Clean a selection of containers or all at once.
    #[structopt(name = "clean")]
    Clean {
        /// Clean selection of containers.
        #[structopt(short, long)]
        all: bool,

        /// Names of containers to clean.
        #[structopt(short, long)]
        names: Vec<String>,
    },

    #[structopt(name = "start")]
    Start {},
}

pub fn run_all_local(cfg: Config) {
    println!("run all jobs local");
    for vol in cfg.local_volumes.iter() {
        println!("run local {:?}", vol.name);
        run_local(&vol.path, &vol.name, &vol.command, &vol.port_map)
    }
}

pub fn run_some_local(cfg: Config, jobs: Vec<String>) {
    let filtered: Vec<LocalVolume> = cfg
        .local_volumes
        .into_iter()
        .filter(|vol| jobs.iter().any(|j| j == &vol.name))
        .collect();

    for vol in filtered.iter() {
        println!("run local {}", vol.name);
        run_local(&vol.path, &vol.name, &vol.command, &vol.port_map)
    }
}

pub fn run_all() {
    println!("run all jobs defined in docker-compose");
    run("docker-compose", vec!["up"]);
}

pub fn clean_all(cfg: Config) {
    for vol in cfg.local_volumes.iter() {
        cleanup(&vol.name);
        println!("cleaned {}", vol.name);
    }
}

pub fn clean_some(cfg: Config, names: Vec<String>) {
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
