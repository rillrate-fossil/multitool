use super::Watcher;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use std::sync::Arc;
use tokio::time::Instant;

impl Watcher {
    pub fn check(&mut self, ctx: &mut Context<Self>) {
        let checker = Checker {
            url: self.url.clone(),
        };
        ctx.spawn_task(checker, (), ());
    }
}

pub struct Report {
    duration: u64,
    // TODO: Add `status`, etc.
}

struct Checker {
    url: Arc<String>,
}

#[async_trait]
impl LiteTask for Checker {
    type Output = Report;

    async fn interruptable_routine(mut self) -> Result<Self::Output, Error> {
        let started = Instant::now();
        let _body = reqwest::get(self.url.as_ref()).await?.text().await?;
        let elapsed = started.elapsed().as_millis();
        let report = Report {
            duration: elapsed as u64,
        };
        Ok(report)
    }
}

#[async_trait]
impl TaskEliminated<Checker, ()> for Watcher {
    async fn handle(
        &mut self,
        _id: IdOf<Checker>,
        _tag: (),
        result: Result<Report, TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        let report = result?;
        self.latency.push(report.duration as f64);
        self.tail
            .log_now("fetch", "", format!("{}ms", report.duration));
        Ok(())
    }
}
