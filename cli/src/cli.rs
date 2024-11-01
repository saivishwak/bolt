use bolt::CompilerBackend;
use clap::Parser;

use crate::{commands, types};

#[derive(Parser)]
#[clap(name = "Bolt")]
#[clap(author = "Sai Vishwak K")]
#[clap(version = "1.0")]
#[clap(
    about = "Scripting Language",
    long_about = "Welcome to Bolt! Language built for learning and educational purpose."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<types::Commands>,
}

impl Cli {
    pub fn init() {
        let cmd: Cli = self::Cli::parse();
        match &cmd.command {
            Some(types::Commands::Start {}) => {
                commands::start();
            }
            Some(types::Commands::Run { path }) => commands::run(path),
            Some(types::Commands::Jit { path, backend }) => {
                let compiler_backend: CompilerBackend = CompilerBackend::from(backend);
                commands::jit(path, &compiler_backend)
            }
            Some(types::Commands::Compile {
                path,
                backend,
                out,
                target,
                bytecode,
            }) => {
                let compiler_backend = CompilerBackend::from(backend);
                commands::compile(path, &compiler_backend, out.to_string(), target, *bytecode)
            }
            None => {
                panic!("Command Not Found");
            }
        }
    }
}
