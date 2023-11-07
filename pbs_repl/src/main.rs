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
 - tree <ID>                                      Show the children recursively of an item
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
        if let Err(e) = match cmd {
            Command::ItemMake(params) => self.handle_item_make(params),
            Command::ItemBuy(params) => self.handle_item_buy(params),
            Command::ItemRelease(params) => self.handle_item_release(params),
            Command::ChildAdd(params) => self.handle_child_add(params),
            Command::ChildDel(params) => self.handle_child_del(params),
            Command::List => self.handle_list(),
            Command::Tree(params) => self.handle_tree(params),
            Command::WhereUsed(params) => self.handle_where_used(params),
            Command::Stock(params) => self.handle_stock(params),
            Command::Exit | Command::Help => Ok(()),
        } {
            eprintln!("ERROR : {:?}", e)
        }
    }

    fn handle_item_make(&mut self, params: ItemMakeParams) -> Result<()> {
        let item = self.store.make_item(&params.name)?;
        println!("  ADDED {item}");
        Ok(())
    }

    fn handle_item_buy(&mut self, params: ItemBuyParams) -> Result<()> {
        let item = self.store.buy_item(&params.pn, &params.name)?;
        println!("  ADDED {item}");
        Ok(())
    }

    fn handle_item_release(&mut self, params: ItemReleaseParams) -> Result<()> {
        let item = self.store.release(params.id)?;
        println!("  RELEASED : {item}");
        Ok(())
    }

    fn handle_list(&self) -> Result<()> {
        for item in self.store.items()? {
            println!("  - {item}");
        }
        Ok(())
    }

    fn handle_child_add(&mut self, params: ChildAddParams) -> Result<()> {
        self.store
            .add_child(params.parent_id, params.child_id, params.quantity)
    }

    fn handle_child_del(&mut self, params: ChildDelParams) -> Result<()> {
        self.store.remove_child(params.parent_id, params.child_id)
    }

    fn print_children(&self, id: i64, level: usize) -> Result<()> {
        let indent = "  ".repeat(level);
        for child in &self.store.children(id)? {
            let item = child.item();
            let quantity = child.quantity();
            println!("{indent}- {item} : quantity={quantity}");
            self.print_children(item.id(), level + 1)?;
        }
        Ok(())
    }

    fn handle_tree(&self, params: TreeParams) -> Result<()> {
        let parent = self.store.item(params.id)?;
        println!("{parent}");
        self.print_children(parent.id(), 1)
    }

    fn handle_where_used(&self, params: WhereUsedParams) -> Result<()> {
        let child = self.store.item(params.id)?;
        let parents = self.store.where_used(params.id)?;
        println!("{child} parents :");
        for item in parents {
            println!("  - {item}");
        }
        Ok(())
    }

    fn handle_stock(&self, params: StockParams) -> Result<()> {
        for child in &self.store.stock(params.id)? {
            println!(
                "  - {item} : {quantity}",
                item = child.item(),
                quantity = child.quantity()
            );
        }
        Ok(())
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
