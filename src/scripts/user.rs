use std::sync::{Arc, Mutex};
use std::error::Error;
// use std::thread;

use crate::scripts::results::QueryResult;

const URLS: &'static [&'static str] = &[
    "hi mom/",
];

pub async fn user(needle: &String) -> Result<(), Box<dyn Error>>{
    let client = reqwest::Client::new();
    let data = Arc::new(
        Mutex::<Vec<QueryResult>>::new(vec![])
    );
    for url in URLS { // this should eventually be computed in parallel
        let req_url = url.to_string() + needle;
        let resp = client.get(&req_url).send().await;

        if let Err(e) = resp { return Err(Box::new(e)) }

        let query_result = QueryResult::new(
            url.to_string(),
            req_url,
            0
        );

        let c_data = Arc::clone(&data);
        let results = &mut *c_data.lock().unwrap();
        results.push(query_result);
        // if resp is not an error, add url to data maybe. or just print ...
    }
    for result in &*data.lock().unwrap() {
        println!("Found needle in {}", result.url);
    }
    Ok(())
}
