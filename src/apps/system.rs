use anyhow::Error;
use rillrate::board::BoardListTracer;
use rillrate::pulse::{Label, PulseFrameSpec, PulseFrameTracer, Range};
use std::collections::HashMap;
use sysinfo::{ProcessorExt, System, SystemExt};
use tokio::time::{sleep, Duration};

const APP: &str = "System";

const D_SYS: &str = "System Monitor";
const D_CPU: &str = "CPU Monitor";

const G_LOAD: &str = "System Load";
const G_INFO: &str = "System Info";
const G_CPUS: &str = "CPUs";

pub async fn run() -> Result<(), Error> {
    let mut system = System::new_all();

    let cpu_board = BoardListTracer::new([APP, D_SYS, G_INFO, "CPU Info"].into());
    let proc = system.global_processor_info();
    cpu_board.set("Frequency", format!("{} MHz", proc.frequency()));
    cpu_board.set("Name", proc.name());
    cpu_board.set("Vendor ID", proc.vendor_id());
    cpu_board.set("Brand", proc.brand());

    let info_board = BoardListTracer::new([APP, D_SYS, G_INFO, "System Info"].into());
    info_board.set("Name", system.name().unwrap_or_default());
    info_board.set(
        "Kernel Version",
        system.kernel_version().unwrap_or_default(),
    );
    info_board.set("Host Name", system.host_name().unwrap_or_default());
    info_board.set("OS Version", system.os_version().unwrap_or_default());
    info_board.set(
        "OS Version (long)",
        system.long_os_version().unwrap_or_default(),
    );
    info_board.set("Total Memory", system.total_memory());
    info_board.set("Total Swap", system.total_swap());

    let cpu_spec = Some(PulseFrameSpec {
        retain: 30,
        range: Range::new(0.0, 100.0),
        label: Label::pct_100(),
    });

    let memory_spec = Some(PulseFrameSpec {
        retain: 30,
        range: Range::new(0.0, system.total_memory() as f32),
        // TODO: Check is that correct? Or 1024x?
        label: Label::new("Gb", 1_000_000.0),
    });

    let swap_spec = Some(PulseFrameSpec {
        retain: 30,
        range: Range::new(0.0, system.total_swap() as f32),
        label: Label::new("Gb", 1_000_000.0),
    });

    let cpu_total =
        PulseFrameTracer::new([APP, D_SYS, G_LOAD, "CPU [total]"].into(), cpu_spec.clone());
    let memory_total =
        PulseFrameTracer::new([APP, D_SYS, G_LOAD, "Memory [total]"].into(), memory_spec);
    let swap_total = PulseFrameTracer::new(
        [APP, D_SYS, G_LOAD, "Swap [total]"].into(),
        swap_spec.clone(),
    );

    let mut cpu_tracers = HashMap::new();
    loop {
        system.refresh_all();
        cpu_total.add(system.global_processor_info().cpu_usage());
        memory_total.add(system.used_memory() as f32);
        swap_total.add(system.used_swap() as f32);

        for (id, proc) in system.processors().iter().enumerate() {
            let cpu_id = id + 1;
            let tracer = cpu_tracers.entry(cpu_id).or_insert_with(|| {
                let name = format!("CPU-{:02}", cpu_id);
                PulseFrameTracer::new([APP, D_CPU, G_CPUS, &name].into(), cpu_spec.clone())
            });
            tracer.add(proc.cpu_usage());
        }

        info_board.set("Uptime", format!("{} secs", system.uptime()));
        sleep(Duration::from_millis(1_000)).await;
    }
}
