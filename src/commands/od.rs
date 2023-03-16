/*
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use url::Url; // , ParseError};
use mailboxvalidator::{validate_email, is_disposable_email};
use serde_json::value;

// mailboxvalidator crate for email checking.
fn verify_email(needle: &String) -> 
    (Result<value::Value, ReqError>, Result<value::Value, ReqError>) {
    (validate_email(needle.as_str(), "API_TOKEN_NEEDED"), is_disposable_email(needle.as_str(), "API_TOKEN_NEEDED"))
}

async fn facebook(client: &reqwest::Client,
                 result: &Arc<Mutex<Vec<String>>>,
                 needle: &String) -> Result<(), reqwest::Error> {

    let i_result = Arc::clone(result);
    let mut i_result = i_result.lock().unwrap();

    let url = Url::parse("https://graph.facebook.com/v8.0/me")?;
    let res = (*client).get(url)
        .query(&[("access_token", "API_TOKEN_NEEDED"), ("fields", "email")])
        .send().await;
    let json: HashMap<String, String> = res?.json()?;

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

    let url = Url::parse("https://api.twitter.com/1.1/account/verify_credentials.json")?;
    let res = (*client).post(url)
        .header("Authorization". "API_TOKEN_NEEDED")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("email={}", needle))
        .send().await;

    match res {
        Ok(r) => {
            // check if email exists
            if r.status().is_success() {
                (*i_result).push("Twitter: email found".to_string());
            } else {
                (*i_result).push("Twitter: email not found".to_string());
            }
        },
        // theoretically should never hit this
        Err(e) => (*i_result).push("Twitter: ERROR".to_string()),
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
    )?;
    let res = (*client).get(url).send().await;
    let json: HashMap<String, String> = res?.json()?;

    // check if email exists
    if json.get("status") == Some(&"fail".to_string()) {
        (*i_result).push("instagram".to_string());
    }

    Ok(())
}

pub async fn email(needle: &String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    let email_status = match verify_email(needle) {
        (Ok(vs), Ok(ds)) => (vs, ds),
        (Err(e), Err(e2)) => return reqwest::Error, // change error type
    };

    instagram(&client, &result, needle).await?;
    twitter(&client, &result, needle).await?;
    facebook(&client, &result, needle).await?;

    Ok(())
}
*/
