use structopt::StructOpt;

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
}
