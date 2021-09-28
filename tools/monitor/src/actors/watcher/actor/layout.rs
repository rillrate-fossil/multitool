use super::*;
use rillrate::basis::Layout;

impl Watcher {
    pub fn register_layout(&mut self) {
        let mut tab = Layout::new(["Monitor", self.name.as_ref()]);
        tab.add_item((0, 0), (60, 40), self.tail_entry.as_ref());
        tab.add_item((60, 0), (40, 40), self.latency_entry.as_ref());
        tab.add_item((0, 60), (20, 30), self.interval_entry.as_ref());
        tab.register();
    }

    pub fn unregister_layout(&mut self) {}
}
