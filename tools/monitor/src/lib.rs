mod actors;

use anyhow::Error;
use clap::Clap;
use meio::System;

#[derive(Clap)]
pub struct Opts {
    pub url: String,
}

pub async fn run(opts: Opts) -> Result<(), Error> {
    let watcher = actors::watcher::Watcher::new(opts.url);
    System::spawn_and_wait(watcher).await;
    Ok(())
}
