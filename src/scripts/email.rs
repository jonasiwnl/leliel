use std::sync::{Arc, Mutex};
use url::Url; // , ParseError};

pub async fn email(needle: &String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    instagram(&client, &result, needle).await?;
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
    let res = (*client).post(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36")
        .send().await;
    if let Err(e) = res { return Err(e); }
    let resp = res.unwrap();
    if resp.status() == 200 {
        let body = resp.text().await?;
        if body.contains("email_is_taken") {
            (*i_result).push("instagram".to_string());
        }
    }
    Ok(())
}
