use meio::Actor;

pub struct Watcher {
    url: String,
}

impl Actor for Watcher {
    type GroupBy = ();
}
