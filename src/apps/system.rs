use anyhow::Error;
use rillrate::prime::*;
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

    let cpu_board = Board::new(
        [APP, D_SYS, G_INFO, "CPU Info"],
        Default::default(),
        BoardOpts::default(),
    );
    let proc = system.global_processor_info();
    cpu_board.set("Frequency", format!("{} MHz", proc.frequency()));
    cpu_board.set("Name", proc.name());
    cpu_board.set("Vendor ID", proc.vendor_id());
    cpu_board.set("Brand", proc.brand());

    let info_board = Board::new(
        [APP, D_SYS, G_INFO, "System Info"],
        Default::default(),
        BoardOpts::default(),
    );
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

    let cpu_opts = PulseOpts::default().retain(30u32).pct_100();
    let memory_opts = PulseOpts::default()
        .retain(30u32)
        .min(0)
        .max(system.total_memory() as f64)
        .suffix("Gb")
        .divisor(1_000_000);
    let swap_opts = PulseOpts::default()
        .retain(30u32)
        .min(0)
        .max(system.total_swap() as f64)
        .suffix("Gb")
        .divisor(1_000_000);

    let cpu_total = Pulse::new(
        [APP, D_SYS, G_LOAD, "CPU [total]"],
        Default::default(),
        cpu_opts.clone(),
    );
    let memory_total = Pulse::new(
        [APP, D_SYS, G_LOAD, "Memory [total]"],
        Default::default(),
        memory_opts,
    );
    let swap_total = Pulse::new(
        [APP, D_SYS, G_LOAD, "Swap [total]"],
        Default::default(),
        swap_opts,
    );

    let mut cpu_tracers = HashMap::new();
    loop {
        system.refresh_all();
        cpu_total.push(system.global_processor_info().cpu_usage());
        memory_total.push(system.used_memory() as f64);
        swap_total.push(system.used_swap() as f64);

        for (id, proc) in system.processors().iter().enumerate() {
            let cpu_id = id + 1;
            let tracer = cpu_tracers.entry(cpu_id).or_insert_with(|| {
                let name = format!("CPU-{:02}", cpu_id);
                Pulse::new(
                    [APP, D_CPU, G_CPUS, &name],
                    Default::default(),
                    cpu_opts.clone(),
                )
            });
            tracer.push(proc.cpu_usage());
        }

        info_board.set("Uptime", format!("{} secs", system.uptime()));
        sleep(Duration::from_millis(1_000)).await;
    }
}
