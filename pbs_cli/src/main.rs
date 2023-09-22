use std::io::{self, Write};

use parser::{AddChildParams, AddParams, StockParams, TreeParams, WhereUsedParams};
use pbs_core::{Result, Store};

use crate::parser::{get_command, Command};

mod parser;

const STORE_URI: &str = "store.db3";

const COMMANDS: &str = r#"
 - help                                           This help
 - exit                                           Exit the pbs CLI
 - add <PART_NUMBER> <NAME>                       Add a item to the store
 - list                                           List all items in the store
 - add-child <PARENT_PN> <CHILD_PN> <QUANTITY>    Add a child item to an parent item
 - tree <PN>                                      Show the children of an item
 - where-used <PN>                                Show all items where the given <PN> is used"#;

struct PbsCli {
    store: Store,
}

impl PbsCli {
    fn new() -> Result<Self> {
        Ok(PbsCli {
            store: Store::open(STORE_URI)?,
        })
    }

    fn prompt(&self) -> io::Result<String> {
        print!("pbs]] ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn handle_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Add(params) => self.handle_add(params),
            Command::List => self.handle_list(),
            Command::AddChild(params) => self.handle_add_child(params),
            Command::Tree(params) => self.handle_tree(params),
            Command::WhereUsed(params) => self.handle_where_used(params),
            Command::Stock(params) => self.handle_stock(params),
            Command::Exit | Command::Help => {}
        }
    }

    fn handle_add(&mut self, params: AddParams) {
        match self.store.new_item(&params.pn, &params.name) {
            Ok(item) => println!("  added {item}"),
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_list(&self) {
        match self.store.get_items() {
            Ok(items) => {
                for item in items {
                    println!("  - {item}");
                }
            }
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_add_child(&mut self, params: AddChildParams) {
        if let Err(e) = self
            .store
            .add_child(&params.parent_pn, &params.child_pn, params.quantity)
        {
            eprintln!("ERROR: {:?}", e);
        }
    }

    fn handle_tree(&self, params: TreeParams) {
        match self.store.get_children(&params.pn) {
            Ok(children) => {
                for (item, quantity) in children {
                    println!("  - {item} : {quantity}");
                }
            }
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }
    fn handle_where_used(&self, params: WhereUsedParams) {
        match self.store.where_used(&params.pn) {
            Ok(parents) => {
                for item in parents {
                    println!("  - {item}");
                }
            }
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_stock(&self, params: StockParams) {
        match self.store.get_stock(&params.pn) {
            Ok(items) => {
                for (item, quantity) in items {
                    println!("  - {item} : {quantity}");
                }
            }
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }
}

fn main() -> Result<()> {
    let mut pbs_cli = PbsCli::new()?;
    loop {
        match pbs_cli.prompt() {
            Ok(input) => match get_command(&input) {
                Ok(cmd) => match cmd {
                    Command::Exit => break,
                    Command::Help => println!("PBS CLI commands: {}", COMMANDS),
                    command => pbs_cli.handle_cmd(command),
                },
                Err(err) => eprintln!("ERROR : {}", err),
            },
            Err(err) => eprintln!("ERROR : {}", err),
        }
    }
    Ok(())
}
