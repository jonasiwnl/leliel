use std::sync::{Arc, Mutex};
use std::error::Error;
// use std::thread;

const URLS: &'static [&'static str] = &[
    "hi mom/",
];

pub fn search(needle: &String) -> Result<(), String>{
    let data = Arc::new(
        Mutex::<Vec<String>>::new(vec![])
    );
    for url in URLS { // this should eventually be computed in parallel
        let req_url = url.to_string() + needle;
        let resp = request(&req_url);

        // if resp is not an error, add url to data maybe. or just print ...
    }
    Ok(())
}

fn request(url: &String) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
