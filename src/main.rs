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

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("\nSTARTING...\n");

    let args = Command::parse();
    let flag = args.flag;
    let needle = args.needle;

    match run(&flag, &needle).await {
        Ok(o) => Ok(o),
        Err(c) => Err(c),
    }
}
