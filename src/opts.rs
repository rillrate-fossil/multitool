use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    System,
    Docker,
}
