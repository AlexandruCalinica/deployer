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
    check_dependencies();

    let _args = Cli::from_args();
    // println!("{:#?}", args.cmd);

    let content = fs::read_to_string("deployer.json").expect("could not read file");

    let _url = match read_from(&content) {
        Ok(res) => {
            res.url;
        }
        Err(err) => println!("{:?}", err),
    };

    run("docker-compose", "up");
}
