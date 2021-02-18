use std::fs;

use structopt::StructOpt;

mod cli;
mod command;
mod config;
mod deps;

use crate::cli::Cli;
use crate::command::run;
use crate::config::read_from;
use crate::deps::check_dependencies;

fn main() {
    match Cli::from_args() {
        Cli::Run { all, jobs, local } => {
            if local == true && jobs.len() == 0 {
                println!("run all local");
            }
            if local == true && jobs.len() > 0 {
                println!("run local {:?}", jobs);
            }
            if all {
                println!("run all");
            }
        }
    }

    check_dependencies();

    let content = fs::read_to_string("deployer.json").expect("could not read file");

    let volumes = match read_from(&content) {
        Ok(res) => {
            res.local_volumes;
        }
        Err(err) => println!("{:?}", err),
    };
    println!("{:?}", volumes);

    run("docker-compose", vec!["up"]);
}
