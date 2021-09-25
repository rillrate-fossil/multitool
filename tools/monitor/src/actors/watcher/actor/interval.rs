use super::Watcher;
use anyhow::Error;
use async_trait::async_trait;
use meio::{ActionHandler, Context};
use rillrate::meio_addon::TracerAction;
use rillrate::prime::slider::SliderState;
use std::time::Duration;

impl Watcher {
    pub fn set_interval_callback(&mut self, ctx: &mut Context<Self>) {
        self.interval.forward(ctx.address().clone());
    }
}

// TODO: Need a special `trait` here?
#[async_trait]
impl ActionHandler<TracerAction<SliderState>> for Watcher {
    async fn handle(
        &mut self,
        msg: TracerAction<SliderState>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        if let Some(action) = msg.envelope.action {
            // TODO: Reschedule site checker
            log::info!("Interval changed: {}", action);
            self.handle.update(Duration::from_secs(action as u64)).ok();
            self.interval.apply(action as f64);
        }
        Ok(())
    }
}
