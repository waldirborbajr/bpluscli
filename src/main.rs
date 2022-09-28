mod utils;

use clap::Parser;
use std::io::Result;
use utils::{Bpluscli, Cli, Command};

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();

    let bplus = Bpluscli::new();

    match command {
        Command::Add { key, val } => bplus.kvadd(&key, &val),
        Command::Del { key } => println!("Del selected {}", key),
        Command::Get { key } => bplus.kvget(&key),
        Command::Lst => println!("List selected "),
    };

    Ok(())
}
