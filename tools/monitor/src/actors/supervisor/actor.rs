use meio::Actor;

pub struct Supervisor {}

impl Actor for Supervisor {
    type GroupBy = ();
}
