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

pub struct QueryResult<'a> {
    username: &'a String,
    url: &'a String,
    user_url: &'a String,
    status: Status,
    query_time: i32,
}

impl<'a> QueryResult<'_> {
    pub fn new(username: &String,
               url: &String,
               user_url: &String,
               status: Status,
               query_time: i32) -> Self {
        QueryResult{ username, url, user_url, status, query_time }
    }
}
