use diesel::{result::Error, Table};
use models::{Userauth, User};

use crate::schema::{userauth::{self, username}, followers, likes, replies, tweets, users, useranalytics};

mod models;
mod schema;
mod connection;


fn main() -> Result<(), Error>{
    println!("Customary hello world!");

    let db = connection::Database::new();

    let user = Userauth {
        email: "adi@123.com".into(),
        user_id: 2,
        password_hash: "fuckyiou".into(),
        username: "uname".into()
    };
    // let output = db.insert_or_update(userauth::table, user);
    dbg!(userauth::table::all_columns());
    dbg!(schema::users::table::all_columns());
    dbg!(schema::likes::table::all_columns());
    dbg!(schema::followers::table::all_columns().0);
    // dbg!(output);
    Ok(())
}