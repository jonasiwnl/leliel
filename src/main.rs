use clap::Parser;
use std::error::Error;
mod scripts;
use crate::scripts::run::run;

#[derive(Parser)]
#[clap(author = "arael", version, about)]
/// a basic osint cli
struct Command {
    flag: String,
    needle: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("\nSTARTING...\n");

    let args = Command::parse();
    let flag = args.flag;
    let needle = args.needle;

    if needle.is_none() {
        println!("No needle provided!");
        return Ok(());
    }

    run(&flag, &needle.unwrap()).await
}
