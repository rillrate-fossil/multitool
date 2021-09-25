use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, System};
use rillrate::prime::*;

pub struct Supervisor {
    input_name: Input,
    input_url: Input,
}

impl Actor for Supervisor {
    type GroupBy = ();
}

impl Supervisor {
    pub fn new() -> Self {
        let input_name = Input::new("global.settings.add.name", InputOpts::default());
        let input_url = Input::new("global.settings.add.url", InputOpts::default());
        Self {
            input_name,
            input_url,
        }
    }
}

#[async_trait]
impl StartedBy<System> for Supervisor {
    async fn handle(&mut self, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<System> for Supervisor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}
