use anyhow::Error;
use rillrate::PulseFrameTracer;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessorExt, System, SystemExt};

const PKG: &str = "Heroic Toys";
const DSHB_SYSTEM: &str = "System Monitor";
const GRP_CPUS: &str = "CPUs";

fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let _handle = rillrate::start();
    let cpu_total = PulseFrameTracer::new([PKG, DSHB_SYSTEM, GRP_CPUS, "CPU [total]"].into(), None);
    let mut system = System::new_all();
    loop {
        system.refresh_cpu();
        let usage = system.global_processor_info().cpu_usage();
        cpu_total.add(usage);
        thread::sleep(Duration::from_millis(500));
    }
}
