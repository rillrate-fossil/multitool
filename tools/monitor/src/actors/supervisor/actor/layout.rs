use super::*;
use rillrate::basis::*;

impl Supervisor {
    pub fn register_layout(&mut self) {
        let mut tab = Layout::new(["Global", "Settings"]);
        tab.set_container(Row::new(vec![
            Expanded::new(
                Column::new(vec![
                    Text::new("Add new check", TextAlign::Center).into(),
                    Flow::new(INPUT_NAME).into(),
                    Flow::new(INPUT_URL).into(),
                    Row::new(vec![Flow::new(BUTTON_ADD).into()]).into(),
                ]),
                1.0,
            )
            .into(),
            Spacer::new(3.0).into(),
        ]));
        tab.register();
    }

    pub fn unregister_layout(&mut self) {}
}
