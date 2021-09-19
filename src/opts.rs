use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Docker,
    Monitor(MonitorOpts),
    System,
}

#[derive(Clap)]
pub struct MonitorOpts {
    pub url: String,
}
