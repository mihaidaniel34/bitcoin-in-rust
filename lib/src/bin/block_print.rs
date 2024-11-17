use std::env;
use std::fs::File;
use std::process::exit;
use lib::types::Block;
use lib::utils::Saveable;

fn main() {
    let path = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        eprintln!("Usage: block_print <file>");
        exit(1)
    };

    if let Ok(file) = File::open(&path) {
        let block = Block::load(file)
            .expect("Failed to load block");
        println!("{:#?}", block)
    }

}