use std::result::Result::Err;

#[path = "./search.rs"] mod search;
#[path = "./help.rs"] mod help;
pub fn run(flag: &String, needle: &String) -> Result<(), String> {
    match flag.as_str() {
        "s" => search::search(needle),
        "h" => help::help(),
        _ => Err("unknown flag.".to_string()),
    }
}
