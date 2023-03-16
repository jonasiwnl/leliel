mod commands;
mod types;

use clap::Parser;
use std::error::Error;
use commands::{email, user};

#[derive(Parser)]
#[clap(author = "arael", version, about)]
/// a basic osint cli
struct Command {
    flag: String,
    needle: Option<String>,
}

pub async fn run(flag: &String, needle: &String) -> Result<(), Box<dyn Error>> {
    match flag.as_str() {
        "u" | "user" | "username" => user::user(needle).await,
        "e" | "email" => email::email(needle).await,
        _ => { 
            println!("Invalid flag!");
            Ok(())
        },
    }
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
