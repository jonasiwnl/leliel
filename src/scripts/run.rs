#[path = "./search.rs"] mod search;

pub async fn run(flag: &String, needle: &String) -> Result<(), reqwest::Error> {
    match flag.as_str() {
        "s" | "search" => search::search(needle).await,
        _ => { 
            eprintln!("invalid flag");
            Ok(())
        },
    }
}
