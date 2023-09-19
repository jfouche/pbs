use std::io::{self, Write};

use parser::{AddChildParams, AddParams};
use pbs_core::{Result, Store};

use crate::parser::{get_command, Command};

mod parser;

const STORE_URI: &str = "store.db3";

trait PbsCli {
    fn handle_cmd(&mut self, command: Command);
    fn handle_add(&mut self, params: AddParams);
    fn handle_list(&self);
    fn handle_add_child(&mut self, params: AddChildParams);
}

impl PbsCli for Store {
    fn handle_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Add(params) => self.handle_add(params),
            Command::List => self.handle_list(),
            Command::AddChild(params) => self.handle_add_child(params),
            _ => {}
        }
    }

    fn handle_add(&mut self, params: AddParams) {
        match self.new_item(&params.pn, &params.name) {
            Ok(item) => println!("  added item {}", item.name),
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_list(&self) {
        {
            match self.get_items() {
                Ok(items) => {
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
                Err(e) => eprintln!("ERROR : {:?}", e),
            }
        }
    }

    fn handle_add_child(&mut self, params: AddChildParams) {
        if let Err(e) = self.add_child(&params.parent_pn, &params.child_pn, params.quantity) {
            eprintln!("ERROR: {:?}", e);
        }
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
                    command => store.handle_cmd(command),
                },
                Err(err) => println!("ERROR : {}", err),
            },
            Err(err) => println!("ERROR : {}", err),
        }
    }
    Ok(())
}
