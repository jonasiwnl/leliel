#[path = "user.rs"] mod user;
#[path = "email.rs"] mod email;

pub async fn run(flag: &String, needle: &String) -> Result<(), reqwest::Error> {
    match flag.as_str() {
        "u" | "user" | "username" => user::user(needle).await,
        "e" | "email" => email::email(needle),
        _ => { 
            eprintln!("invalid flag");
            Ok(())
        },
    }
}
