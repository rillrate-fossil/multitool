use super::*;
use rillrate::basis::{Layout, LayoutTab};

impl Watcher {
    pub fn register_layout(&mut self) {
        let mut layout = Layout::new(self.name.as_ref());
        let mut tab = LayoutTab::new("Latency");
        tab.add_item((0, 0), (20, 30), LATENCY.get().unwrap().as_ref());
        tab.add_item((0, 30), (20, 30), TAIL.get().unwrap().as_ref());
        tab.add_item((0, 60), (20, 30), INTERVAL.get().unwrap().as_ref());
        layout.add_tab(tab);
        layout.register();
    }

    pub fn unregister_layout(&mut self) {}
}
