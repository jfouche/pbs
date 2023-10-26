use std::io::{self, Write};

use parser::{
    ChildAddParams, ChildDelParams, ItemBuyParams, ItemMakeParams, ItemReleaseParams, StockParams,
    TreeParams, WhereUsedParams,
};
use pbs_core::{Result, Store};

use crate::parser::{get_command, Command};

mod parser;

const STORE_URI: &str = "store.db3";

const COMMANDS: &str = r#"
 - help                                           This help
 - exit                                           Exit the pbs REPL
 - item make <NAME>                               Create a "make" item, allocating a PN
 - item buy <PART_NUMBER> <NAME>                  Create a "Buy" item, with it's external PN
 - list                                           List all items in the store
 - child add <PARENT_ID> <CHILD_ID> <QUANTITY>    Add a child item to an parent item
 - child del <PARENT_ID> <CHILD_ID>               Remove a child item from a parent item
 - tree <ID>                                      Show the children of an item
 - where-used <ID>                                Show all items where the given <PN> is used"#;

struct PbsRepl {
    store: Store,
}

impl PbsRepl {
    fn new() -> Result<Self> {
        Ok(PbsRepl {
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
            Command::ItemMake(params) => self.handle_item_make(params),
            Command::ItemBuy(params) => self.handle_item_buy(params),
            Command::ItemRelease(params) => self.handle_item_release(params),
            Command::ChildAdd(params) => self.handle_child_add(params),
            Command::ChildDel(params) => self.handle_child_del(params),
            Command::List => self.handle_list(),
            Command::Tree(params) => self.handle_tree(params),
            Command::WhereUsed(params) => self.handle_where_used(params),
            Command::Stock(params) => self.handle_stock(params),
            Command::Exit | Command::Help => {}
        }
    }

    fn handle_item_make(&mut self, params: ItemMakeParams) {
        match self.store.make_item(&params.name) {
            Ok(item) => println!("  ADDED {item}"),
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_item_buy(&mut self, params: ItemBuyParams) {
        match self.store.buy_item(&params.pn, &params.name) {
            Ok(item) => println!("  ADDED {item}"),
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_item_release(&mut self, params: ItemReleaseParams) {
        match self.store.release(params.id) {
            Ok(item) => println!("  RELEASED : {item}"),
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_list(&self) {
        match self.store.items() {
            Ok(items) => {
                for item in items {
                    println!("  - {item}");
                }
            }
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_child_add(&mut self, params: ChildAddParams) {
        if let Err(e) = self
            .store
            .add_child(params.parent_id, params.child_id, params.quantity)
        {
            eprintln!("ERROR: {:?}", e);
        }
    }

    fn handle_child_del(&mut self, params: ChildDelParams) {
        if let Err(e) = self.store.remove_child(params.parent_id, params.child_id) {
            eprintln!("ERROR: {:?}", e);
        }
    }

    fn handle_tree(&self, params: TreeParams) {
        match self.store.item(params.id) {
            Ok(parent) => match self.store.children(params.id) {
                Ok(children) => {
                    println!("{parent} childrens :");
                    for (item, quantity) in children {
                        println!("  - {item} : {quantity}");
                    }
                }
                Err(e) => eprintln!("ERROR : {:?}", e),
            },
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_where_used(&self, params: WhereUsedParams) {
        match self.store.item(params.id) {
            Ok(child) => match self.store.where_used(params.id) {
                Ok(parents) => {
                    println!("{child} parents :");
                    for item in parents {
                        println!("  - {item}");
                    }
                }
                Err(e) => eprintln!("ERROR : {:?}", e),
            },
            Err(e) => eprintln!("ERROR : {:?}", e),
        }
    }

    fn handle_stock(&self, params: StockParams) {
        match self.store.stock(params.id) {
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
    let mut pbs_repl = PbsRepl::new()?;
    loop {
        match pbs_repl.prompt() {
            Ok(input) => match get_command(&input) {
                Ok(cmd) => match cmd {
                    Command::Exit => break,
                    Command::Help => println!("PBS REPL commands: {}", COMMANDS),
                    command => pbs_repl.handle_cmd(command),
                },
                Err(err) => eprintln!("ERROR : {}", err),
            },
            Err(err) => eprintln!("ERROR : {}", err),
        }
    }
    Ok(())
}
