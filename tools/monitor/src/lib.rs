mod actors;

use anyhow::Error;
use clap::Clap;
use rillrate::prime::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration, Instant};

#[derive(Clap)]
pub struct Opts {
    pub url: String,
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

    let slider = Slider::new(
        "lab.monitor.settings.interval",
        SliderOpts::default()
            .label("Slide Me!")
            .min(1)
            .max(60)
            .step(1),
    );
    let this = slider.clone();
    // TODO: Implement instant change of the interval
    this.apply(10);
    let interval = Arc::new(AtomicU64::new(10));
    let interval_ref = interval.clone();
    slider.sync_callback(move |envelope| {
        if let Some(action) = envelope.action {
            interval_ref.store(action as u64, Ordering::Relaxed);
            this.apply(action);
        }
        Ok(())
    });

    loop {
        let started = Instant::now();
        let _body = reqwest::get(&opts.url).await?.text().await?;
        let elapsed = started.elapsed().as_millis();
        latency.push(elapsed as f64);
        live_tail.log_now("fetch", "", format!("{}ms", elapsed));
        let secs = interval.load(Ordering::Relaxed);
        sleep(Duration::from_secs(secs)).await;
    }
}
