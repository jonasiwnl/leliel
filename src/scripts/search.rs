use std::sync::{Arc, Mutex};
// use std::thread;

use crate::scripts::results::{QueryResult, Status};

const URLS: &'static [&'static str] = &[
    "hi mom/",
];

pub async fn search(needle: &String) -> Result<(), reqwest::Error>{
    let client = reqwest::Client::new();
    let data = Arc::new(
        Mutex::<Vec<String>>::new(vec![])
    );
    for url in URLS { // this should eventually be computed in parallel
        let req_url = url.to_string() + needle;
        let resp = client.get(&req_url).send().await;

        if let Err(e) = resp { return Err(e) }

        let query_result = QueryResult::new(&needle, 
                                                        &url.to_string(), 
                                                        &req_url, 
                                                        Status::DNE, 0);

        let c_data = Arc::clone(&data);
        let found_urls = &mut *c_data.lock().unwrap();
        found_urls.push(req_url);
        // if resp is not an error, add url to data maybe. or just print ...
    }
    for url in &*data.lock().unwrap() {
        println!("Found needle in {}", url);
    }
    Ok(())
}
