mod cmdline;

use cmdline::*;
use pbs_core::{Result, Store};

fn main() {
    match parse_cmd_line(std::env::args()) {
        Ok(command) => match command {
            Commands::Usage => usage(),
            Commands::Add(params) => {
                println!("  CMD ADD: {:?}", params);
                if let Err(err) = handle_cmd_add(params) {
                    eprintln!("ERROR : {:?}", err);
                }
            }
        },
        Err(err) => {
            eprintln!("ERROR : {:?}", err);
            usage();
        }
    }
}

fn usage() {
    eprintln!("Usage");
}

fn handle_cmd_add(params: AddParams) -> Result<()> {
    let mut store = Store::open("store.db3")?;
    let mut item = store.new_item()?;
    item.pn = params.pn;
    item.name = params.name;
    store.save_item(item)?;
    Ok(())
}
