mod actors;

use anyhow::Error;
use meio::System;

pub async fn run() -> Result<(), Error> {
    let watcher = actors::supervisor::Supervisor::new();
    System::spawn_and_wait(watcher).await;
    Ok(())
}
