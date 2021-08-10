use anyhow::Error;
use rillrate::board::BoardListTracer;
use shiplift::Docker;
use tokio::time::{sleep, Duration};

const APP: &str = "Docker";

const D_INFO: &str = "Docker Info";
const D_STAT: &str = "Docker Stats";

const G_INFO: &str = "Generic Info";

fn y_n(yes: bool) -> &'static str {
    if yes {
        "Yes"
    } else {
        "No"
    }
}

pub async fn run() -> Result<(), Error> {
    let info_board = BoardListTracer::new([APP, D_INFO, G_INFO, "Info"].into());
    let resr_board = BoardListTracer::new([APP, D_INFO, G_INFO, "Resources"].into());
    let docker = Docker::new();
    match docker.info().await {
        Ok(info) => {
            // TODO: Fix width
            //info_board.set("Id", info.id);
            info_board.set("Driver", info.driver);
            info_board.set("Kernel Version", info.kernel_version);
            resr_board.set("OS", info.operating_system);

            resr_board.set("Containers", info.containers);
            resr_board.set("Images", info.images);
            resr_board.set("CPUs", info.n_cpu);
            resr_board.set("Memory (total)", info.mem_total);
            resr_board.set("Memory (limit)", y_n(info.memory_limit));
            resr_board.set("Swap (limit)", y_n(info.swap_limit));
        }
        Err(err) => {
            log::error!("Can't get docker into: {}", err);
            // TODO: Use Alert here!
        }
    }
    loop {
        sleep(Duration::from_millis(1_000)).await;
    }
}
