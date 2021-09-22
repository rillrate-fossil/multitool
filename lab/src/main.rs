mod opts;

use anyhow::Error;
use clap::Clap;
use opts::{Opts, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::parse();
    env_logger::try_init()?;
    match opts.subcmd {
        SubCommand::Docker => {
            rillrate::install("app")?;
            mtl_docker::run().await
        },
        SubCommand::Monitor(opts) => {
            // TODO: Imporve that (don't call `install` for every method)
            mtl_monitor::prepare();
            rillrate::install("app")?;
            mtl_monitor::run(opts).await
        },
        SubCommand::System => {
            rillrate::install("app")?;
            mtl_system::run().await
        },
    }
}
