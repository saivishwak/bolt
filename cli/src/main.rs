pub mod cli;
pub mod commands;
pub mod repl;
pub mod types;

fn main() {
    println!(
        r###"
   ___       _ _       __                                          
  / __\ ___ | | |_    / /  __ _ _ __   __ _ _   _  __ _  __ _  ___ 
 /__\/// _ \| | __|  / /  / _` | '_ \ / _` | | | |/ _` |/ _` |/ _ \
/ \/  \ (_) | | |_  / /__| (_| | | | | (_| | |_| | (_| | (_| |  __/
\_____/\___/|_|\__| \____/\__,_|_| |_|\__, |\__,_|\__,_|\__, |\___|
                                      |___/             |___/      
    
    Welcome to Bolt! Language built for learning and educational purpose.
    "###
    );
    cli::Cli::init();
}
