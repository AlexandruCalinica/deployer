use structopt::StructOpt;

#[structopt(
    name = "Youlo deployer",
    about = "CLI tool for orchestrating docker deployments."
)]
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// Run a selection of containers or all at once.
    #[structopt(subcommand)]
    pub run: Run,
}

#[derive(Debug, StructOpt)]
pub enum Run {
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
}
