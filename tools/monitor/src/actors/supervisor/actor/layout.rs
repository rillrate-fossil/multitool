use super::*;
use rillrate::basis::Layout;

impl Supervisor {
    pub fn register_layout(&mut self) {
        let mut tab = Layout::new(["Global", "Settings"]);
        tab.add_item((0, 0), (20, 30), INPUT_NAME);
        tab.add_item((0, 30), (20, 30), INPUT_URL);
        tab.add_item((0, 60), (20, 30), BUTTON_ADD);
        tab.register();
    }

    pub fn unregister_layout(&mut self) {}
}
