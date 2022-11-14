pub enum Status {
    EXISTS,
    DNE,
    ILLEGAL,
    ERR,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::EXISTS => "this username already exists.".to_string(),
            Status::DNE => "this username is available.".to_string(),
            Status::ILLEGAL => "this username isn't allowed.".to_string(),
            Status::ERR => "an error occured.".to_string(),
            _ => "".to_string(),
        }
    }
}

pub struct QueryResult {
    pub url: String,
    pub user_url: String,
    pub status: Status,
    query_time: i32,
}

impl QueryResult {
    pub fn new(url: String,
               user_url: String,
               status: Status,
               query_time: i32) -> Self {
        QueryResult{ url, user_url, status, query_time }
    }
}
