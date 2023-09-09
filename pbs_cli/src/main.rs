use std::io::{self, Write};

use parser::AddParams;
use pbs_core::{Result, Store};

use crate::parser::{get_command, Command};

mod parser;

#[derive(PartialEq)]
enum CliState {
    Continue,
    Exit,
}

fn main() -> Result<()> {
    loop {
        print!("pbs> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match get_command(&input) {
                Ok((_, cmd)) => {
                    if handle_command(cmd) == CliState::Exit {
                        break;
                    }
                }
                Err(err) => println!("ERROR : {}", err),
            },
            Err(err) => println!("ERROR : {}", err),
        }
    }
    Ok(())
}

fn handle_command(cmd: Command) -> CliState {
    let mut state = CliState::Continue;
    match cmd {
        // add ...
        Command::Add(params) => match handle_add(&params) {
            Ok(()) => {
                println!("Added item {pn}", pn = params.pn)
            }
            Err(err) => println!("ERROR : {err:?}"),
        },
        // list ...
        Command::List => {}
        // exit ...
        Command::Exit => state = CliState::Exit,
    }
    state
}

fn handle_add(params: &AddParams) -> Result<()> {
    let mut store = Store::open("store.db3")?;
    let item = store.new_item(&params.pn, &params.name)?;
    Ok(())
}
