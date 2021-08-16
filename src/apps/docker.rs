use anyhow::Error;
use futures::StreamExt;
use rillrate::board::Board;
use rillrate::pulse::{Pulse, PulseSpec};
use rillrate::range::{Label, Range};
use shiplift::{ContainerListOptions, Docker};
use std::collections::HashMap;
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
    let info_board = Board::new([APP, D_INFO, G_INFO, "Global Info"]);
    let resr_board = Board::new([APP, D_INFO, G_INFO, "Resources"]);
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
    struct Group {
        board: Board,
        memory: Pulse,
        cpu: Pulse,
    }
    let mut groups_pool: HashMap<String, Group> = HashMap::new();
    loop {
        let mut groups: HashMap<String, Group> = HashMap::new();
        let containers = docker.containers();
        let list = containers.list(&ContainerListOptions::default()).await?;
        for cont in list {
            log::warn!("{:?}", cont);
            let name = cont.names.get(0).unwrap_or(&cont.id);
            //let stats = cont.stats();
            let g: Group;
            if let Some(group) = groups_pool.remove(&cont.id) {
                g = group;
            } else {
                // Creates a new tracer for a new container
                let memory_spec = PulseSpec {
                    retain: 30,
                    range: Range::new(0.0, 10_000_000.0),
                    // TODO: Check is that correct? Or 1024x?
                    label: Label::new("Gb", 1_000_000.0),
                };
                let memory = Pulse::new([APP, D_STAT, name, "Memory"], memory_spec);

                let cpu_spec = PulseSpec {
                    retain: 30,
                    range: Range::new(0.0, 100.0),
                    label: Label::pct_100(),
                };
                let cpu = Pulse::new([APP, D_STAT, name, "CPU"], cpu_spec);

                let board = Board::new([APP, D_STAT, name, "Info"]);
                g = Group { board, memory, cpu };
            }
            g.board.set("Image", cont.image);
            g.board.set("Command", cont.command);
            g.board.set("State", cont.state);
            g.board.set("Status", cont.status);

            // TODO: Spawn a `LiteTask` to collect stats.
            let opt_stats = containers.get(&cont.id).stats().next().await;
            log::warn!("STATS of {}: {:?}", name, opt_stats);
            if let Some(Ok(stats)) = opt_stats {
                log::warn!("Usage: {}", stats.memory_stats.usage);
                g.memory.push(stats.memory_stats.usage as f64);
            }

            // TODO: Add ports forwards
            groups.insert(cont.id, g);
        }
        groups_pool = groups;
        sleep(Duration::from_millis(1_000)).await;
    }
}
