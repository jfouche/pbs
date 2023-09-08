use std::io::{self, Write};

use pbs_core::{Result, Store};

use crate::parser::{get_command, Command};

mod parser;

fn main() -> Result<()> {
    loop {
        print!("pbs> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match get_command(&input) {
                Ok((_, cmd)) => match cmd {
                    Command::Add { pn } => match add_item(&pn) {
                        Ok(()) => {
                            println!("Added item {pn}")
                        }
                        Err(err) => println!("ERROR : {err:?}"),
                    },
                    Command::List => {}
                    Command::Exit => break,
                },
                Err(e) => {}
            },
            Err(error) => println!("error: {}", error),
        }
    }
    Ok(())
}

fn add_item(pn: &str) -> Result<()> {
    let mut store = Store::open("store.db3")?;
    let mut item = store.new_item(pn)?;
    item.pn = pn.to_string();
    store.save_item(item)?;
    Ok(())
}
