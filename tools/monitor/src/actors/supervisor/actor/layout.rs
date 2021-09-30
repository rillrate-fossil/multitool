use super::*;
use rillrate::basis::*;

impl Supervisor {
    pub fn register_layout(&mut self) {
        let mut tab = Layout::new(["Global", "Settings"]);
        tab.set_container(Row::new(vec![Column::new(vec![
            Flow::new(INPUT_NAME).into(),
            Flow::new(INPUT_URL).into(),
            Row::new(vec![Flow::new(BUTTON_ADD).into()]).into(),
        ])
        .into()]));
        tab.register();
    }

    pub fn unregister_layout(&mut self) {}
}
