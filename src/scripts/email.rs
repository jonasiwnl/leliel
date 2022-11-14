

pub fn email(needle: &String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    Ok(())
}

async fn instagram(client: reqwest::Client, needle: &String) -> bool {
    let url = "https://www.instagram.com/accounts/web_create_ajax/attempt/";
    let resp = client.post(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36")
        .send().await;
    false
}
