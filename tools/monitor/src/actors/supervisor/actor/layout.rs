use super::*;
use rillrate::basis::{Layout, LayoutTab};

impl Supervisor {
    pub fn register_layout(&mut self) {
        let mut layout = Layout::new("Global");
        let mut tab = LayoutTab::new("Settings");
        tab.add_item((0, 0), (20, 30), INPUT_NAME);
        tab.add_item((0, 30), (20, 30), INPUT_URL);
        tab.add_item((0, 60), (20, 30), BUTTON_ADD);
        layout.add_tab(tab);
        layout.register();
    }

    pub fn unregister_layout(&mut self) {}
}
