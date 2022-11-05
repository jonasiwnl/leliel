use std::sync::{Arc, Mutex};
// use std::thread;

const URLS: &'static [&'static str] = &[
    "hi mom/",
];

pub fn search(needle: &String) -> Result<(), String>{
    let data = Arc::new(
        Mutex::<Vec<String>>::new(vec![])
    );
    for url in URLS {
        let req_url = url.to_string() + needle;
        println!("{}", req_url);
    }
    Ok(())
}
