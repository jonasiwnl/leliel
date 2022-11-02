use std::result::Result::Err;

#[path = "./search.rs"] mod search;

pub fn run(flag: &String, needle: &String) -> Result<(), String> {
    match flag.as_str() {
        "s" => search::search(needle),
        _ => Err("unknown flag.".to_string()),
    }
}
