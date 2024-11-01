use bolt::CompilerBackend;
use clap::{Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Backend {
    LLVM,
}

impl From<&Backend> for CompilerBackend {
    fn from(value: &Backend) -> Self {
        match value {
            Backend::LLVM => CompilerBackend::LLVM,
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Start {},
    Run {
        #[clap(short, long)]
        path: String,
    },
    Jit {
        #[clap(short, long)]
        path: String,
        #[clap(short, long)]
        backend: Backend,
    },
    Compile {
        #[clap(short, long)]
        path: String,
        #[clap(short, long)]
        backend: Backend,
        #[clap(short, long)]
        out: String,
        #[clap(short, long)]
        target: String,
        #[clap(short = 'y', long, default_value = "false")]
        bytecode: bool,
    },
}
