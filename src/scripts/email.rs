use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use url::Url; // , ParseError};
use mailboxvalidator::{validate_email, is_disposable_email};

// mailboxvalidator crate for email checking.
fn verify_email(needle: &String) -> (bool, bool) {
    (validate_email(needle), is_disposable_email(needle))
}

async fn facebook(client: &reqwest::Client,
                 result: &Arc<Mutex<Vec<String>>>,
                 needle: &String) -> Result<(), reqwest::Error> {

    let i_result = Arc::clone(result);
    let mut i_result = i_result.lock().unwrap();

    let url = Url::parse("https://graph.facebook.com/v8.0/me");
    let res = (*client).get(url)
        .query(&[("access_token", "API_TOKEN_NEEDED"), ("fields", "email")])
        .send().await;
    let json: HashMap<String, String> = res.json()?;

    if json.get("email") == Some(&email) {
        (*i_result).push("facebook".to_string());
    }

    Ok(())
}

async fn twitter(client: &reqwest::Client,
                 result: &Arc<Mutex<Vec<String>>>,
                 needle: &String) -> Result<(), reqwest::Error> {

    let i_result = Arc::clone(result);
    let mut i_result = i_result.lock().unwrap();

    let url = Url::parse("https://api.twitter.com/1.1/account/verify_credentials.json");
    let res = (*client).post(url)
        .header("Authorization". "API_TOKEN_NEEDED")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("email={}", email))
        .send().await;

    // check if email exists
    if res.status().is_success() {
        (*i_result).push("twitter".to_string());
    }

    Ok(())
}

async fn instagram(client: &reqwest::Client,
                   result: &Arc<Mutex<Vec<String>>>,
                   needle: &String) -> Result<(), reqwest::Error> {

    let i_result = Arc::clone(result);
    let mut i_result = i_result.lock().unwrap();

    let url = Url::parse_with_params(
        "https://www.instagram.com/accounts/web_create_ajax/attempt/",
        &[("email", needle)]
    ).unwrap();
    let res = (*client).get(url).send().await;
    let json: HashMap<String, String> = res.json()?;

    // check if email exists
    if json.get("status") == Some(&"fail".to_string()) {
        (*i_result).push("instagram".to_string());
    }

    Ok(())
}

pub async fn email(needle: &String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    let email_status = verify_email(needle);

    instagram(&client, &result, needle).await?;

    Ok(())
}
