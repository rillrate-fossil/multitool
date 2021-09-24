use anyhow::Error;
use async_trait::async_trait;
use meio::{Action, ActionHandler, Actor, Context, StartedBy, System};
use rillrate::prime::*;
use tokio::time::{sleep, Duration, Instant};

pub struct Watcher {
    url: String,
    latency: Pulse,
    tail: LiveTail,
    interval: Slider,
}

impl Actor for Watcher {
    type GroupBy = ();
}

impl Watcher {
    pub fn new(url: String) -> Self {
        let latency = Pulse::new(
            "lab.monitor.latency.pulse",
            Default::default(),
            PulseOpts::default()
                .retain(30u32)
                .min(0)
                .max(1_000)
                .higher(true)
                .suffix("ms"),
        );
        let tail = LiveTail::new(
            "lab.monitor.latency.events",
            Default::default(),
            LiveTailOpts::default(),
        );
        let interval = Slider::new(
            "lab.monitor.settings.interval",
            SliderOpts::default()
                .label("Slide Me!")
                .min(1)
                .max(60)
                .step(1),
        );
        Self {
            url,
            latency,
            tail,
            interval,
        }
    }
}

#[async_trait]
impl StartedBy<System> for Watcher {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.set_interval_callback(ctx);
        Ok(())
    }
}

struct ChangeInterval(u64);

impl Action for ChangeInterval {}

impl Watcher {
    fn set_interval_callback(&mut self, ctx: &mut Context<Self>) {
        let addr = ctx.address().clone();
        self.interval.async_callback(move |envelope| {
            let mut addr = addr.clone();
            async move {
                if let Some(action) = envelope.action {
                    let msg = ChangeInterval(action as u64);
                    addr.act(msg).await
                } else {
                    Ok(())
                }
            }
        });
    }
}

#[async_trait]
impl ActionHandler<ChangeInterval> for Watcher {
    async fn handle(&mut self, msg: ChangeInterval, _ctx: &mut Context<Self>) -> Result<(), Error> {
        // TODO: Reschedule site checker
        self.interval.apply(msg.0 as f64);
        Ok(())
    }
}

impl Watcher {
    async fn check(&mut self) -> Result<(), Error> {
        let started = Instant::now();
        let _body = reqwest::get(&self.url).await?.text().await?;
        let elapsed = started.elapsed().as_millis();
        self.latency.push(elapsed as f64);
        self.tail.log_now("fetch", "", format!("{}ms", elapsed));
        Ok(())
    }
}
