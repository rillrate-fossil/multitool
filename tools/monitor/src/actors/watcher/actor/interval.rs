use super::Watcher;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Action, ActionHandler, Context};

struct ChangeInterval(u64);

impl Action for ChangeInterval {}

impl Watcher {
    pub fn set_interval_callback(&mut self, ctx: &mut Context<Self>) {
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
