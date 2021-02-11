use structopt::StructOpt;

#[structopt(
    name = "Youlo deployer",
    about = "CLI tool for orchestrating docker deployments."
)]
#[derive(StructOpt)]
pub struct Cli {
    /// Run multiple or a single docker container based on the docker-compose.yml file.
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Run(Run),
}

#[derive(Debug, StructOpt)]
pub struct Run {
    #[structopt(long, short)]
    all: bool,

    #[structopt(long, short)]
    local: String,
}
