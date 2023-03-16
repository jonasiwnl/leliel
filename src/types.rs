pub struct QueryResult {
    pub url: String,
    pub user_url: String,
    query_time: i32,
}

impl QueryResult {
    pub fn new(url: String,
               user_url: String,
               query_time: i32) -> Self {
        QueryResult{ url, user_url, query_time }
    }
}

enum LelielError {
    
}

