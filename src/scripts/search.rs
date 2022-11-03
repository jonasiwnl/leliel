use std::sync::{Arc, Mutex};
use std::thread;

pub fn search(needle: &String) -> Result<(), String>{
    let data = Arc::new(
        Mutex::<Vec<String>>::new(vec![])
    );
    println!("{}", needle);
    Ok(())
}
