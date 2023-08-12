use chrono::Utc;
use serde::{Serialize, Deserialize};
use regex::Regex;


#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UserAuth {
    pub userid: usize,
    pub email: Option<String>,
    pub username: String,
    pub password_hash: String
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub userid: i64,
    pub username: String,
    pub handle: String,
    pub is_private: bool,
    pub bio: Option<String>,
    pub profile_pic: Option<String>,
    pub banner: Option<String>,
    pub date_created: chrono::DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Tweet {
    pub tweetid: i32,
    pub created_at: chrono::NaiveDateTime,
    pub userid: i32,
    pub content: String,
    // pub retweet_id: Option<i32>,
    // pub quote_tweet_id: Option<i32>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Like {
    pub user_id: i32,
    pub tweet_id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Reply {
    pub reply_id: i32,
    pub tweet_id: i32,
    pub user_id: i32,
    pub reply_content: String,
    pub date_created: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UserAnalytics {
    pub user_id: i32,
    pub date: chrono::NaiveDateTime,
    pub time_spent_on_app: i32,
}

#[derive(Debug)]
pub struct AllowedMethods {
    pub pattern: Regex,
    pub methods: Vec<String>,
}
