mod checker;
mod heartbeat;
mod interval;
mod layout;

use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::task::HeartBeatHandle;
use meio::{Actor, Context, InterruptedBy, StartedBy};
use once_cell::sync::OnceCell;
use rillrate::prime::*;
use std::sync::Arc;
use std::time::Duration;

// TODO: Use local fields instead
static LATENCY: OnceCell<String> = OnceCell::new();
static TAIL: OnceCell<String> = OnceCell::new();
static INTERVAL: OnceCell<String> = OnceCell::new();

pub struct Watcher {
    name: String,
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
        LATENCY.set(format!("site.{}.latency.pulse", name)).unwrap();
        let latency = Pulse::new(
            LATENCY.get().unwrap().as_ref(),
            Default::default(),
            PulseOpts::default()
                .retain(30u32)
                .min(0)
                .max(1_000)
                .higher(true)
                .suffix("ms"),
        );
        TAIL.set(format!("site.{}.latency.events", name)).unwrap();
        let tail = LiveTail::new(
            TAIL.get().unwrap().as_ref(),
            Default::default(),
            LiveTailOpts::default(),
        );
        INTERVAL
            .set(format!("site.{}.settings.interval", name))
            .unwrap();
        let interval = Slider::new(
            INTERVAL.get().unwrap().as_ref(),
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
