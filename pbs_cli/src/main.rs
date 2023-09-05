use std::env::Args;

mod cmdline;

use cmdline::*;

fn main() {
    match parse_cmd_line(std::env::args()) {
        Ok(command) => match command {
            Commands::Usage => usage(),
            Commands::Add(params) => {
                println!("  CMD ADD: {:?}", params);
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

fn handle_cmd_add(args: Args) {}
