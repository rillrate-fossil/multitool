mod add_url;
mod layout;

use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, System};
use rillrate::prime::*;

const INPUT_NAME: &str = "global.settings.add.name";
const INPUT_URL: &str = "global.settings.add.url";
const BUTTON_ADD: &str = "global.settings.add.button";

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
        let input_name = Input::new(INPUT_NAME, InputOpts::default().label("Name"));
        let input_url = Input::new(INPUT_URL, InputOpts::default().label("URL"));
        let button_add = Click::new(BUTTON_ADD, ClickOpts::default().label("Add"));
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
        self.register_layout();
        self.listen_adds(ctx);
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<System> for Supervisor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.unregister_layout();
        ctx.shutdown();
        Ok(())
    }
}
