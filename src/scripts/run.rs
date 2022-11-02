use std::result::Result::Err;

pub fn run(flag: &String) -> Result<(), String> {
    match *flag {
        _ => Err("unknown flag.".to_string())
    }
}
