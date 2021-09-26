use super::*;
use rillrate::basis::{Layout, LayoutTab};

impl Watcher {
    pub fn register_layout(&mut self) {
        let mut layout = Layout::new("Monitor");
        let mut tab = LayoutTab::new(self.name.as_ref());
        tab.add_item((0, 0), (60, 40), self.tail_entry.as_ref());
        tab.add_item((60, 0), (40, 40), self.latency_entry.as_ref());
        tab.add_item((0, 60), (20, 30), self.interval_entry.as_ref());
        layout.add_tab(tab);
        layout.register();
    }

    pub fn unregister_layout(&mut self) {}
}
