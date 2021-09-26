mod checker;
mod heartbeat;
mod interval;
mod layout;

use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::task::HeartBeatHandle;
use meio::{Actor, Context, InterruptedBy, StartedBy};
use rillrate::prime::*;
use std::sync::Arc;
use std::time::Duration;

pub struct Watcher {
    name: String,
    url: Arc<String>,
    latency: Pulse,
    tail: LiveTail,
    interval: Slider,
    handle: HeartBeatHandle,

    latency_entry: String,
    tail_entry: String,
    interval_entry: String,
}

impl Actor for Watcher {
    type GroupBy = ();
}

impl Watcher {
    pub fn new(name: String, url: String) -> Self {
        let latency_entry = format!("site.{}.latency.pulse", name);
        let latency = Pulse::new(
            latency_entry.as_ref(),
            Default::default(),
            PulseOpts::default()
                .retain(30u32)
                .min(0)
                .max(1_000)
                .higher(true)
                .suffix("ms"),
        );
        let tail_entry = format!("site.{}.latency.events", name);
        let tail = LiveTail::new(
            tail_entry.as_ref(),
            Default::default(),
            LiveTailOpts::default(),
        );
        let interval_entry = format!("site.{}.settings.interval", name);
        let interval = Slider::new(
            interval_entry.as_ref(),
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
            name,
            url: Arc::new(url),
            latency,
            tail,
            interval,
            handle,
            latency_entry,
            tail_entry,
            interval_entry,
        }
    }
}

#[async_trait]
impl StartedBy<Supervisor> for Watcher {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.register_layout();
        self.spawn_heartbeat(ctx);
        self.set_interval_callback(ctx);
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<Supervisor> for Watcher {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.unregister_layout();
        ctx.shutdown();
        Ok(())
    }
}
