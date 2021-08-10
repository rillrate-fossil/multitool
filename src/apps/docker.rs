use anyhow::Error;
use rillrate::board::BoardListTracer;
use std::thread;
use std::time::Duration;

const APP: &str = "Docker";

const D_INFO: &str = "Docker Info";
const D_STAT: &str = "Docker Stats";

const G_INFO: &str = "Generic Info";

pub fn run() -> Result<(), Error> {
    let docker_board = BoardListTracer::new([APP, D_INFO, G_INFO, "Parameters"].into());
    loop {
        thread::sleep(Duration::from_millis(700));
    }
}
