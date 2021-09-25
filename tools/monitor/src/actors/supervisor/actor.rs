mod add_url;

use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, System};
use rillrate::prime::*;

pub struct Supervisor {
    input_name: Input,
    name: String,
    input_url: Input,
    url: String,
    button_add: Click,
}

impl Actor for Supervisor {
    type GroupBy = ();
}

impl Supervisor {
    pub fn new() -> Self {
        let input_name = Input::new("global.settings.add.name", InputOpts::default());
        let input_url = Input::new("global.settings.add.url", InputOpts::default());
        let button_add = Click::new("global.settings.add.button", ClickOpts::default());
        Self {
            input_name,
            name: String::new(),
            input_url,
            url: String::new(),
            button_add,
        }
    }
}

#[async_trait]
impl StartedBy<System> for Supervisor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.listen_adds(ctx);
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
