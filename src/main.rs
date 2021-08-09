use anyhow::Error;
use rillrate::PulseFrameTracer;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessorExt, System, SystemExt};

const PKG: &str = "Heroic Toys";
const DSHB_SYSTEM: &str = "System Monitor";
const GRP_LOAD: &str = "System Load";
const GRP_CPUS: &str = "CPUs";

fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let _handle = rillrate::start();
    let cpu_total = PulseFrameTracer::new([PKG, DSHB_SYSTEM, GRP_LOAD, "CPU [total]"].into(), None);
    let memory_total =
        PulseFrameTracer::new([PKG, DSHB_SYSTEM, GRP_LOAD, "Memory [total]"].into(), None);
    let swap_total =
        PulseFrameTracer::new([PKG, DSHB_SYSTEM, GRP_LOAD, "Swap [total]"].into(), None);
    //let cpu_map = HashMap::new();
    let mut system = System::new_all();
    loop {
        system.refresh_all();
        cpu_total.add(system.global_processor_info().cpu_usage());
        memory_total.add(system.used_memory() as f32);
        swap_total.add(system.used_swap() as f32);
        thread::sleep(Duration::from_millis(700));
    }
}
