use anyhow::Error;
use rillrate::board::BoardListTracer;
use tokio::time::{sleep, Duration};

const APP: &str = "Docker";

const D_INFO: &str = "Docker Info";
const D_STAT: &str = "Docker Stats";

const G_INFO: &str = "Generic Info";

pub async fn run() -> Result<(), Error> {
    let docker_board = BoardListTracer::new([APP, D_INFO, G_INFO, "Parameters"].into());
    loop {
        sleep(Duration::from_millis(1_000)).await;
    }
}
