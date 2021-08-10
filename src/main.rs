mod apps;
mod opts;

use anyhow::Error;
use clap::Clap;
use opts::{Opts, SubCommand};

fn main() -> Result<(), Error> {
    let opts = Opts::parse();
    env_logger::try_init()?;
    let _handle = rillrate::start();
    match opts.subcmd {
        SubCommand::System => apps::system::run(),
        SubCommand::Docker => apps::docker::run(),
    }
}
