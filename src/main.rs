use std::env::{args, Args};
use std::process;

mod scripts;
use crate::scripts::run::run;

fn main() {
    println!("\nSTARTING...\n");

    let mut args: Args = args();
    let flag = args.nth(1).unwrap_or_else(|| {
        eprintln!("ERROR: enter a flag.");
        process::exit(1);
    });
    let needle = args.nth(0).unwrap_or_else(|| {
        eprintln!("ERROR: enter a needle.");
        process::exit(1);
    });

    match run(&flag, &needle) {
        Ok(o) => o,
        Err(c) => println!("ERROR: {}", c),
    }
}
