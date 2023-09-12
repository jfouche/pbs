use std::io::{self, Write};

use pbs_core::{Result, Store};

use crate::parser::{get_command, Command};

mod parser;

const STORE_URI: &str = "store.db3";

trait PbsCli {
    fn handle_cmd(&mut self, command: Command) -> Result<()>;
}

impl PbsCli for Store {
    fn handle_cmd(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            // add ...
            Command::Add(params) => {
                let item = self.new_item(&params.pn, &params.name)?;
                println!("  added item {}", item.name)
            }
            // list ...
            Command::List => {
                let items = self.get_items()?;
                // Get the max size of PN
                let max_pn_len = items.iter().map(|i| i.pn.len()).max().unwrap_or(0);
                for item in items {
                    println!(
                        "  - item {pn:>w$}\t{name}",
                        pn = item.pn,
                        name = item.name,
                        w = max_pn_len
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut store = Store::open(STORE_URI)?;
    loop {
        print!("pbs> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match get_command(&input) {
                Ok((_, cmd)) => match cmd {
                    Command::Exit => break,
                    command => store.handle_cmd(command)?,
                },
                Err(err) => println!("ERROR : {}", err),
            },
            Err(err) => println!("ERROR : {}", err),
        }
    }
    Ok(())
}
