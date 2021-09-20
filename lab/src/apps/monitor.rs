use crate::opts::MonitorOpts;
use anyhow::Error;
use rillrate::prime::*;
use tokio::time::{sleep, Duration, Instant};

pub async fn run(opts: MonitorOpts) -> Result<(), Error> {
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
    loop {
        let started = Instant::now();
        let body = reqwest::get(&opts.url).await?.text().await?;
        let elapsed = started.elapsed().as_millis();
        latency.push(elapsed as f64);
        sleep(Duration::from_secs(10)).await;
    }
    Ok(())
}
