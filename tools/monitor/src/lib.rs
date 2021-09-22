use anyhow::Error;
use clap::Clap;
use rillrate::prime::*;
use tokio::time::{sleep, Duration, Instant};

#[derive(Clap)]
pub struct Opts {
    pub url: String,
}

// TODO: Use a channed instead to notify `ConfigWatcher`
pub fn prepare() {
    rate_config::embed_config!();
}

pub async fn run(opts: Opts) -> Result<(), Error> {
    // TODO: Add many workers?
    let latency_opts = PulseOpts::default()
        .retain(30u32)
        .min(0)
        .max(1_000)
        .higher(true)
        .suffix("ms");
    let latency = Pulse::new(
        "lab.monitor.latency.pulse",
        Default::default(),
        latency_opts,
    );

    let live_tail_opts = LiveTailOpts::default();
    let live_tail = LiveTail::new(
        "lab.monitor.latency.events",
        Default::default(),
        live_tail_opts,
    );

    loop {
        let started = Instant::now();
        let body = reqwest::get(&opts.url).await?.text().await?;
        let elapsed = started.elapsed().as_millis();
        latency.push(elapsed as f64);
        live_tail.log_now("fetch", "", format!("{}ms", elapsed));
        sleep(Duration::from_secs(10)).await;
    }
    Ok(())
}
