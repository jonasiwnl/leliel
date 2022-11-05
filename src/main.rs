use clap::Parser;

mod scripts;
use crate::scripts::run::run;

#[derive(Parser)]
#[clap(author = "arael", version, about)]
/// a basic osint cli
struct Command {
    flag: String,
    #[clap(default_value_t = ("".to_string()), short, long)]
    needle: String,
}

fn main() {
    println!("\nSTARTING...\n");

    let args = Command::parse();
    let flag = args.flag;
    let needle = args.needle;

    match run(&flag, &needle) {
        Ok(o) => o,
        Err(c) => println!("ERROR: {}", c),
    }
}
