use structopt::StructOpt;

#[structopt(
    name = "Youlo deployer",
    about = "CLI tool for orchestrating docker deployments."
)]
#[derive(StructOpt, Debug)]
pub enum Cli {
    /// Run a selection of containers or all at once.
    #[structopt(name = "run")]
    Run {
        /// Array of containers ready to run.
        #[structopt(short, long, required_if("all", "false"))]
        jobs: Vec<String>,
        /// Run all at once.
        #[structopt(short, long)]
        all: bool,
        /// Run in development mode.
        #[structopt(short, long)]
        local: bool,
    },
}
