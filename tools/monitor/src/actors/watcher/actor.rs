mod checker;
mod heartbeat;
mod interval;

use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::task::HeartBeatHandle;
use meio::{Actor, Context, InterruptedBy, StartedBy};
use rillrate::prime::*;
use std::sync::Arc;
use std::time::Duration;

pub struct Watcher {
    url: Arc<String>,
    latency: Pulse,
    tail: LiveTail,
    interval: Slider,
    handle: HeartBeatHandle,
}

impl Actor for Watcher {
    type GroupBy = ();
}

impl Watcher {
    pub fn new(name: String, url: String) -> Self {
        let latency = Pulse::new(
            format!("site.{}.latency.pulse", name),
            Default::default(),
            PulseOpts::default()
                .retain(30u32)
                .min(0)
                .max(1_000)
                .higher(true)
                .suffix("ms"),
        );
        let tail = LiveTail::new(
            format!("site.{}.latency.events", name),
            Default::default(),
            LiveTailOpts::default(),
        );
        let interval = Slider::new(
            format!("site.{}.settings.interval", name),
            SliderOpts::default()
                .label("Slide Me!")
                .min(1)
                .max(60)
                .step(1),
        );
        const VALUE: u64 = 10;
        let handle = HeartBeatHandle::new(Duration::from_secs(VALUE));
        interval.apply(VALUE as f64);
        Self {
            url: Arc::new(url),
            latency,
            tail,
            interval,
            handle,
        }
    }
}

#[async_trait]
impl StartedBy<Supervisor> for Watcher {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.spawn_heartbeat(ctx);
        self.set_interval_callback(ctx);
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<Supervisor> for Watcher {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}
