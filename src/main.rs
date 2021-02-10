// use fs::read_to_string;
use std::{
    // env::args,
    fs,
    // io::BufReader,
    // path::PathBuf,
    vec,
};
// use structopt::StructOpt;
mod command;
mod config;
mod deps;

use crate::command::run;
use crate::config::read_from;
use crate::deps::Dependency;

/// Search for a pattern
// #[derive(StructOpt)]
// struct Cli {
//     /// The pattern to look for
//     pattern: String,
//     /// The path to the file to read
//     #[structopt(parse(from_os_str))]
//     path: PathBuf,
// }

fn main() {
    // let args = Cli::from_args();

    let content = fs::read_to_string("deployer.json").expect("could not read file");

    match read_from(&content) {
        Ok(res) => println!("{:?}", res.url),
        Err(err) => println!("{:?}", err),
    };

    let mut commands = vec![];
    commands.insert(0, "curl".to_string());
    commands.insert(0, "node".to_string());
    commands.insert(0, "docker".to_string());
    commands.insert(0, "plm".to_string());

    for name in commands.iter() {
        Dependency {
            name: name.to_string(),
        }
        .check();
    }

    run("docker-compose", "up");
}
