use std::sync::{Arc, Mutex};
// use std::thread;

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
        let resp = client.get(req_url).send().await;

        if let Err(e) = resp { return Err(e) }

        // data.push("")
        // if resp is not an error, add url to data maybe. or just print ...
    }
    Ok(())
}
