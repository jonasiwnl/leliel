pub enum Status {
    EXISTS,
    DNE,
    ILLEGAL,
    ERR,
}

impl Status {
    pub fn to_string() -> String {
        match self {
            Status::EXISTS => return "this username already exists.",
            Status::DNE => return "this username is available.",
            Status::ILLEGAL => return "this username isn't allowed.",
            Status::ERR => return "an error occured.",
            _ => todo!(),
        }
    }
}

pub struct Result {
    username: String,
    url: String,
    user_url: String,
    status: Status,
    query_time: i32,
}

impl Result {
    pub fn new(username: String,
               url: String,
               user_url: String,
               status: Status,
               query_time: i32) -> Self {
        Result{ username, url, user_url, status, query_time }
    }
}
