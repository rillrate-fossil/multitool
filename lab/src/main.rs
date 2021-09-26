mod opts;

use anyhow::Error;
use clap::Clap;
use opts::{Opts, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::parse();
    env_logger::try_init()?;
    rillrate::install("app")?;
    match opts.subcmd {
        SubCommand::Docker => mtl_docker::run().await,
        SubCommand::Monitor => mtl_monitor::run().await,
        SubCommand::System => mtl_system::run().await,
    }
}
