use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::error::Error;
use url::Url;

async fn instagram(
    client: &reqwest::Client,
    result: &Arc<Mutex<Vec<String>>>,
    needle: &String
) -> Result<(), Box<dyn Error>> {
    
    let i_result = Arc::clone(result);
    let mut i_result = i_result.lock().unwrap();

    let url = Url::parse_with_params(
        "https://www.instagram.com/accounts/web_create_ajax/attempt/",
        &[("email", needle)]
    )?;

    let res = client.get(url).send().await?;
    let json: HashMap<String, String> = res.json().await?;

    // check if account with email exists
    if json.get("status") == Some(&"fail".to_string()) {
        i_result.push("instagram".to_string());
    }

    Ok(())
}

pub async fn email(needle: &String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    instagram(&client, &result, needle).await?;

    Ok(())
}
