use crate::models::{AllowedMethods, Like, Reply, Tweet, User, UserAuth};
use chrono::Utc;
use regex::Regex;
use std::collections::HashMap;

impl User {
    pub fn new(
        userid: i64,
        username: String,
        handle: String,
        bio: Option<String>,
        profile_pic: Option<String>,
        banner: Option<String>,
        date_created: chrono::DateTime<Utc>,
        is_private: bool,
    ) -> Self {
        Self {
            userid,
            username,
            handle,
            bio,
            profile_pic,
            banner,
            date_created,
            is_private,
        }
    }
}

impl UserAuth {
    pub fn new(
        userid: usize,
        email: Option<String>,
        username: String,
        password_hash: String,
    ) -> Self {
        Self {
            userid,
            email,
            username,
            password_hash,
        }
    }
}

impl Tweet {
    pub fn new(
        tweetid: i32,
        created_at: chrono::NaiveDateTime,
        userid: i32,
        content: String,
    ) -> Self {
        Self {
            tweetid,
            created_at,
            userid,
            content,
        }
    }
}

impl Like {
    pub fn new(user_id: i32, tweet_id: i32) -> Self {
        Self { user_id, tweet_id }
    }
}

impl Reply {
    pub fn new(
        reply_id: i32,
        tweet_id: i32,
        user_id: i32,
        reply_content: String,
        date_created: chrono::NaiveDateTime,
    ) -> Self {
        Self {
            reply_id,
            tweet_id,
            user_id,
            reply_content,
            date_created,
        }
    }
}

impl AllowedMethods {
    pub fn new(regex: String, methods: Vec<String>) -> Self {
        Self {
            pattern: Regex::new(&regex).unwrap(),
            methods,
        }
    }
}
