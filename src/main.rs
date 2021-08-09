use anyhow::Error;
use rillrate::pulse::{PulseFrameSpec, PulseFrameTracer, Range};
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

    let cpu_spec = Some(PulseFrameSpec {
        retain: 30,
        range: Range::new(0.0, 100.0),
    });

    let memory_spec = Some(PulseFrameSpec {
        retain: 30,
        range: Range::min(0.0),
    });

    let cpu_total = PulseFrameTracer::new(
        [PKG, DSHB_SYSTEM, GRP_LOAD, "CPU [total]"].into(),
        cpu_spec.clone(),
    );
    let memory_total = PulseFrameTracer::new(
        [PKG, DSHB_SYSTEM, GRP_LOAD, "Memory [total]"].into(),
        memory_spec.clone(),
    );
    let swap_total = PulseFrameTracer::new(
        [PKG, DSHB_SYSTEM, GRP_LOAD, "Swap [total]"].into(),
        memory_spec.clone(),
    );
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
