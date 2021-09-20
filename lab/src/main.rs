mod apps;
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
        SubCommand::Monitor(opts) => apps::monitor::run(opts).await,
        SubCommand::System => mtl_system::run().await,
    }
}
