use std::env::{args, Args};

mod scripts;
use crate::scripts::run::run;

fn main() {
    let mut args: Args = args();

    println!("\nSTARTING...\n");

    let flag = args.nth(1).unwrap_or(
        "ERROR: enter a process. run 'h' for help.".to_string());

    match run(&flag) {
        Ok(e) => e,
        Err(c) => println!("ERROR: {}", c),
    }
}
