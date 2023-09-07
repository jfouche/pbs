use pbs_core::{Result, Store};

use clap::{arg, Command};

fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let pn = sub_matches
                .get_one::<String>("PART_NUMBER")
                .expect("required");
            add_item(pn)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn cli() -> Command {
    // https://docs.rs/clap/latest/clap/_derive/_cookbook/git/index.html
    Command::new("pbs")
        .about("Product Breakdown Store")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new item to store")
                .arg(arg!(<PART_NUMBER> "Part number"))
                .arg_required_else_help(true),
        )
}

fn add_item(pn: &str) -> Result<()> {
    let mut store = Store::open("store.db3")?;
    let mut item = store.new_item()?;
    item.pn = pn.to_string();
    store.save_item(item)?;
    Ok(())
}
