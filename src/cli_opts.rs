use clap::Clap;

#[derive(Clap)]
pub struct CliOpts {
    #[clap(long)]
    pub period: u64,
    #[clap(long)]
    pub port: String,
    #[clap(long)]
    pub connect: Option<String>,
}
