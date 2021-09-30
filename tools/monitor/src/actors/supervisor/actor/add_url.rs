use super::Supervisor;
use crate::actors::watcher::Watcher;
use anyhow::Error;
use async_trait::async_trait;
use meio::{ActionHandler, Context, Eliminated, IdOf};
use rillrate::meio_addon::TracerAction;
use rillrate::prime::click::ClickState;
use rillrate::prime::input::InputState;

#[derive(Debug, Clone)]
enum InputTag {
    Name,
    Url,
}

impl Supervisor {
    pub fn listen_adds(&mut self, ctx: &mut Context<Self>) {
        self.input_name.forward(InputTag::Name, ctx);
        self.input_url.forward(InputTag::Url, ctx);
        self.button_add.forward((), ctx);
    }
}

#[async_trait]
impl ActionHandler<TracerAction<InputState, InputTag>> for Supervisor {
    async fn handle(
        &mut self,
        msg: TracerAction<InputState, InputTag>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        if let Some(action) = msg.envelope.action {
            match msg.tag {
                InputTag::Name => {
                    self.name = action.clone();
                    self.input_name.apply(action);
                }
                InputTag::Url => {
                    self.url = action.clone();
                    self.input_url.apply(action);
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ActionHandler<TracerAction<ClickState>> for Supervisor {
    async fn handle(
        &mut self,
        msg: TracerAction<ClickState>,
        ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        if let Some(_action) = msg.envelope.action {
            let watcher = Watcher::new(self.name.clone(), self.url.clone());
            ctx.spawn_actor(watcher, ());
        }
        Ok(())
    }
}

#[async_trait]
impl Eliminated<Watcher> for Supervisor {
    async fn handle(&mut self, _id: IdOf<Watcher>, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }
}
