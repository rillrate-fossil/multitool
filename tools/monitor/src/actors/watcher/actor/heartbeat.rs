use super::Watcher;
use anyhow::Error;
use async_trait::async_trait;
use meio::task::{HeartBeat, OnTick, Tick};
use meio::Context;

impl Watcher {
    pub fn spawn_heartbeat(&mut self, ctx: &mut Context<Self>) {
        let heartbeat = HeartBeat::new_with_handle(&self.handle, ctx.address().clone());
        ctx.spawn_task(heartbeat, (), ());
    }
}

#[async_trait]
impl OnTick for Watcher {
    async fn tick(&mut self, _tick: Tick, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.check(ctx);
        Ok(())
    }

    async fn done(&mut self, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }
}
