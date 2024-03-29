// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::offset::Utc;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Deserialize, Serialize};
use crate::schema::{followers, likes, replies, tweets, useranalytics, userauth, users};



#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(primary_key(user_id, follower_id))]
#[diesel(table_name = followers)]
pub struct Follower {
    pub user_id: i32,
    pub follower_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(primary_key(user_id, tweet_id))]
#[diesel(table_name = likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Like {
    pub user_id: i32,
    pub tweet_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(primary_key(reply_id))]
#[diesel(table_name = replies)]
pub struct Reply {
    pub reply_id: i32,
    pub tweet_id: Option<i32>,
    pub user_id: Option<i32>,
    pub reply_content: String,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(primary_key(tweet_id))]
#[diesel(table_name = tweets)]
pub struct Tweet {
    pub tweet_id: i32,
    pub user_id: Option<i32>,
    pub content: String,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
}

#[derive(Queryable, Debug, Identifiable, Default)]
#[diesel(primary_key(user_id, date))]
#[diesel(table_name = useranalytics)]
pub struct Useranalytic {
    pub user_id: i32,
    pub date: DateTime<Utc>,
    pub time_spent_on_app: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = userauth)]
pub struct Userauth {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub handle: String,
    pub is_private: bool,
    pub bio: Option<String>,
    pub profile_pic: Option<String>,
    pub banner: Option<String>,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>
}

