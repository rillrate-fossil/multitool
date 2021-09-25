mod actors;

use anyhow::Error;
use clap::Clap;
use meio::System;

#[derive(Clap)]
pub struct Opts {
    pub url: String,
}

pub async fn run(opts: Opts) -> Result<(), Error> {
    let watcher = actors::supervisor::Supervisor::new();
    System::spawn_and_wait(watcher).await;
    Ok(())
}
