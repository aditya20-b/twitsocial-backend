use regex::Regex;
use crate::handler::PostgresPool;
use dotenv::dotenv;
pub struct Database {
    pub connection: PostgresPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let manager = 
    }
}



#[derive(Debug)]
pub struct AllowedMethods {
    pub pattern: Regex,
    pub methods: Vec<String>,
}
