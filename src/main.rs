use std::{fs, path};

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
    check_dependencies();

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
            println!("all = {:?}, jobs = {:?}, local = {:?}", all, jobs, local);
        }
    }

    let content = fs::read_to_string("deployer.json").expect("could not read file");

    match read_from(&content) {
        Ok(res) => {
            println!("{:?}", res.local_volumes);
        }
        Err(err) => println!("{:?}", err),
    };

    // run("docker-compose", vec!["up"]);
    run_local("./packages/module-1", "module1test");
}

pub fn run_local(target: &str, name: &str) {
    let target_path = path::PathBuf::from(target);
    let absolute_target_path = fs::canonicalize(&target_path).unwrap();

    let cmd = format!(
        r#"run -d --name {} -v {:#?}:/app node:alpine"#,
        name, absolute_target_path
    );

    let splitted: Vec<&str> = cmd.split(' ').collect();

    println!("{:?}", splitted);

    // run("docker", splitted);
}
