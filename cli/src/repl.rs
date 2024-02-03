use bolt::evaluator::{environment::Environment, evaluator::eval};
use std::io::{self, Write};

pub fn start_repl() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut environment = Environment::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let _read_ok = match stdin.read_line(&mut user_input) {
            Ok(_input) => {
                //
            }
            Err(err) => {
                return Err(err);
            }
        };
        let evaluated = eval(user_input.clone(), &mut environment).unwrap();
        println!("{}", evaluated.inspect());
        user_input.clear();
    }
}
