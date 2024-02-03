use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Start {},
    Run {
        #[clap(short, long)]
        path: String,
    },
}
