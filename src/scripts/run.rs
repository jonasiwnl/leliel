use std::error::Error;
#[path = "user.rs"] mod user;
#[path = "email.rs"] mod email;

pub async fn run(flag: &String, needle: &String) -> Result<(), Box<dyn Error>> {
    match flag.as_str() {
        "u" | "user" | "username" => user::user(needle).await,
        "e" | "email" => email::email(needle).await,
        _ => { 
            println!("Invalid flag!");
            Ok(())
        },
    }
}
