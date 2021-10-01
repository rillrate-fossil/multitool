use super::*;
use rillrate::basis::*;

impl Watcher {
    pub fn register_layout(&mut self) {
        let mut tab = Layout::new(["Monitor", self.name.as_ref()]);
        tab.set_container(Column::new(vec![
            Row::new(vec![
                Flow::new(self.tail_entry.as_ref()).into(),
                Flow::new(self.latency_entry.as_ref()).into(),
            ])
            .into(),
            Flow::new(self.interval_entry.as_ref()).into(),
        ]));
        tab.register();
    }

    pub fn unregister_layout(&mut self) {}
}
