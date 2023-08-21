use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;
use diesel::result::Error;
use dotenv::dotenv;
use crate::models::{Userauth, User, Tweet, Like, Follower, Reply};
use crate::schema::{userauth, users, tweets, likes, followers, replies};



pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

// This type defines the actual connection objects generated from the pool
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


pub struct Database {
    _connection: PostgresPool,
}

pub trait BasicDBOperations {
    fn get(&self, id: i32) -> Result<User, Error>;
    fn create(&self, user: User) -> Result<usize, Error>;
    fn update(&self, user: User) -> Result<usize, Error>;
    fn delete(&self, id: i32) -> Result<usize, Error>;
}


/*
List of everything that needs to be implemented:
- new
- create or update userauth
- create or update users
- create or update tweets
- create or update replies
- add likes
- remove likes
- add followers
- remove followers
- get user by id
- get tweet by id
- get reply by id
- get likes by user id
- get followers by user id
- get replies by tweet id
- get tweets by user id
- delete user by id
- delete tweet by id
- delete reply by id

*/
impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder().build(manager).expect("Failed to create pool.");
        Self {
            _connection: pool,
        }
    }

    pub fn get_connection(&self) -> PgPooledConnection {
        self._connection.get().expect("Failed to get connection from pool.")
    }

    pub fn create_or_update_userauth(&self, userauth: Userauth) -> Result<usize, Error> {
        diesel::insert_into(userauth::table)
            .values(&userauth)
            .on_conflict(userauth::user_id)
            .do_update()
            .set(&userauth)
            .execute(&mut self.get_connection())
            
    }

    pub fn get_userauth_by_id(&self, user_id_input: i32) -> Result<Userauth, Error> {
        userauth::table 
            .filter(userauth::user_id.eq(user_id_input))
            .first::<Userauth>(&mut self.get_connection())
    }

    pub fn delete_userauth_by_id(&self, user_id_input:i32) -> Result<usize, Error> {
        diesel::delete(userauth::table)
            .filter(userauth::user_id.eq(user_id_input))
            .execute(&mut self.get_connection())
    }

    pub fn create_or_update_user(&self, user: User) -> Result<usize, Error> {
        diesel::insert_into(users::table)
            .values(&user)
            .on_conflict(users::user_id)
            .do_update()
            .set(&user)
            .execute(&mut self.get_connection())
    }

    pub fn get_user_by_id(&self, user_id_input: i32) -> Result<User, Error> {
        users::table 
            .filter(users::user_id.eq(user_id_input))
            .first::<User>(&mut self.get_connection())
    }

    pub fn delete_user_by_id(&self, user_id_input:i32) -> Result<usize, Error> {
        diesel::delete(users::table)
            .filter(users::user_id.eq(user_id_input))
            .execute(&mut self.get_connection())
    }

    pub fn create_or_update_tweet(&self, tweet: Tweet) -> Result<usize, Error> {
        diesel::insert_into(tweets::table)
            .values(&tweet)
            .on_conflict(tweets::tweet_id)
            .do_update()
            .set(&tweet)
            .execute(&mut self.get_connection())
    }

    pub fn get_tweet_by_id(&self, tweet_id_input: i32) -> Result<Tweet, Error> {
        tweets::table 
            .filter(tweets::tweet_id.eq(tweet_id_input))
            .first::<Tweet>(&mut self.get_connection())
    }

    pub fn delete_tweet_by_id(&self, tweet_id_input:i32) -> Result<usize, Error> {
        diesel::delete(tweets::table)
            .filter(tweets::tweet_id.eq(tweet_id_input))
            .execute(&mut self.get_connection())
    }

    pub fn create_or_update_reply(&self, reply: Reply) -> Result<usize, Error> {
        diesel::insert_into(replies::table)
            .values(&reply)
            .on_conflict(replies::reply_id)
            .do_update()
            .set(&reply)
            .execute(&mut self.get_connection())
    }

    pub fn get_reply_by_id(&self, reply_id_input: i32) -> Result<Reply, Error> {
        replies::table 
            .filter(replies::reply_id.eq(reply_id_input))
            .first::<Reply>(&mut self.get_connection())
    }

    pub fn delete_reply_by_id(&self, reply_id_input:i32) -> Result<usize, Error> {
        diesel::delete(replies::table)
            .filter(replies::reply_id.eq(reply_id_input))
            .execute(&mut self.get_connection())
    }


    pub fn add_like(&self, like: Like) -> Result<usize, Error> {
        diesel::insert_into(likes::table)
            .values(&like)
            .execute(&mut self.get_connection())
    }

    pub fn remove_like(&self, like: Like) -> Result<usize, Error> {
        diesel::delete(likes::table)
            .filter(likes::user_id.eq(like.user_id))
            .filter(likes::tweet_id.eq(like.tweet_id))
            .execute(&mut self.get_connection())
    }

    pub fn add_follower(&self, follower: Follower) -> Result<usize, Error> {
        diesel::insert_into(followers::table)
            .values(&follower)
            .execute(&mut self.get_connection())
    }

    pub fn remove_follower(&self, follower: Follower) -> Result<usize, Error> {
        diesel::delete(followers::table)
            .filter(followers::user_id.eq(follower.user_id))
            .filter(followers::follower_id.eq(follower.follower_id))
            .execute(&mut self.get_connection())
    }

    pub fn get_likes_by_user_id(&self, user_id_input: i32) -> Result<Vec<Like>, Error> {
        likes::table 
            .filter(likes::user_id.eq(user_id_input))
            .load::<Like>(&mut self.get_connection())
    }

    pub fn get_followers_by_user_id(&self, user_id_input: i32) -> Result<Vec<Follower>, Error> {
        followers::table 
            .filter(followers::user_id.eq(user_id_input))
            .load::<Follower>(&mut self.get_connection())
    }

    pub fn get_replies_by_tweet_id(&self, tweet_id_input: i32) -> Result<Vec<Reply>, Error> {
        replies::table 
            .filter(replies::tweet_id.eq(tweet_id_input))
            .load::<Reply>(&mut self.get_connection())
    }

    pub fn get_tweets_by_user_id(&self, user_id_input: i32) -> Result<Vec<Tweet>, Error> {
        tweets::table 
            .filter(tweets::user_id.eq(user_id_input))
            .load::<Tweet>(&mut self.get_connection())
    }

    pub fn delete_like_by_user_id(&self, user_id_input:i32) -> Result<usize, Error> {
        diesel::delete(likes::table)
            .filter(likes::user_id.eq(user_id_input))
            .execute(&mut self.get_connection())
    }

    pub fn delete_follower_by_user_id(&self, user_id_input:i32) -> Result<usize, Error> {
        diesel::delete(followers::table)
            .filter(followers::user_id.eq(user_id_input))
            .execute(&mut self.get_connection())
    }

    pub fn delete_reply_by_tweet_id(&self, tweet_id_input:i32) -> Result<usize, Error> {
        diesel::delete(replies::table)
            .filter(replies::tweet_id.eq(tweet_id_input))
            .execute(&mut self.get_connection())
    }

    pub fn delete_tweet_by_user_id(&self, user_id_input:i32) -> Result<usize, Error> {
        diesel::delete(tweets::table)
            .filter(tweets::user_id.eq(user_id_input))
            .execute(&mut self.get_connection())
    }

    pub fn delete_like_by_tweet_id(&self, tweet_id_input:i32) -> Result<usize, Error> {
        diesel::delete(likes::table)
            .filter(likes::tweet_id.eq(tweet_id_input))
            .execute(&mut self.get_connection())
    }

}
