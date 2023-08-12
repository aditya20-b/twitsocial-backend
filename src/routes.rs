use crate::models::{AllowedMethods, Like, Reply, Tweet, User, UserAnalytics, UserAuth};
use actix_web::{HttpRequest, HttpResponse, Responder, Result, Error};
use chrono::Utc;
use sqlx::{query_as, sqlite::*, Connection, Sqlite, SqlitePool, Row, Column};

pub async fn index() -> Result<HttpResponse, Error> {
    let start = std::time::Instant::now();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "heartbeat": "OK",
        // Have the server return the time taken for the request to be processed
        "time_taken": format!("{}μs", start.elapsed().as_micros()),
    })))
}


// pub async fn get_user(req: HttpRequest) -> impl Responder {
//     let userid: i64 = req.match_info().get("userid").unwrap().parse().unwrap();
//     dbg!(userid);
//     let mut conpool = SqlitePool::connect("sqlite://database.db").await.unwrap();
//     // let query = sqlx::query("SELECT * FROM USERS WHERE userid = ?")
//     // .bind(userid)
//     // .fetch_one(&mut conn)
//     // .await
//     // .unwrap();
//     let q = sqlx::query("SELECT * FROM USERS WHERE userid = ?");
//     // dbg!(q);
//     let query = q.bind(userid);

//     let user = match query.fetch_one(&conpool).await {
//     Ok(user) => user,
//     Err(err) => {
//         // Handle the error here
//         return HttpResponse::InternalServerError().body(format!("Failed to fetch user: {}", err));
//         }

//     };
//     for col in user.columns() {
//         dbg!(col.name());
//         let userstuff: String = user.get(col.name());
//         dbg!(userstuff);
//     };

//     HttpResponse::Ok().body("ok then")

// }


pub async fn get_user(req: HttpRequest) -> impl Responder {
    let userid: i64 = req.match_info().get("userid").unwrap().parse().unwrap();
    let conpool = SqlitePool::connect("sqlite://database.db").await.unwrap();
    let user = sqlx::query_as::<Sqlite, User>(r#"SELECT * FROM USERS WHERE userid = $1"#)
        .bind(userid)
        .fetch_one(&conpool)
        .await
        .unwrap();
    HttpResponse::Ok().json(user)
}

// pub async fn get_user(req: HttpRequest) -> impl Responder {
//     let userid: i32 = req.match_info().get("userid").unwrap().parse().unwrap();
//     let user = User::new(
//         userid,
//         "username".to_string(),
//         "handle".to_string(),
//         None,
//         None,
//         None,
//         chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
//         false,
//     );
//     HttpResponse::Ok().json(user)
// }

async fn get_tweet(req: HttpRequest) -> impl Responder {
    let tweetid: i32 = req.match_info().get("tweetid").unwrap().parse().unwrap();
    let tweet = Tweet::new(
        tweetid,
        chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
        0,
        "content".to_string(),
    );
    HttpResponse::Ok().json(tweet)
}

async fn get_like(req: HttpRequest) -> Result<HttpResponse, Error> {
    let userid: i32 = req.match_info().get("userid").unwrap().parse().unwrap();
    let tweetid: i32 = req.match_info().get("tweetid").unwrap().parse().unwrap();
    let like = Like::new(userid, tweetid);
    Ok(HttpResponse::Ok().json(like))
}


// tfw you realise that this entire """smart implementation""" is dumb asf
fn allowed_methods() -> Vec<AllowedMethods> {
    vec![
        // \d+ matches one or more digits and \w+ matches one or more word characters
        AllowedMethods::new(
            r"^/api/$".to_string(),
            vec!["GET".to_string(), "POST".to_string()],
        ),
        AllowedMethods::new(r"^/api/user/\d+$".to_string(), vec!["GET".to_string()]),
        AllowedMethods::new(r"^/api/tweet/\d+$".to_string(), vec!["GET".to_string()]),
        AllowedMethods::new(r"^/api/like/\d+/\d+$".to_string(), vec!["GET".to_string()]),
    ]
}

pub async fn method_not_allowed_handler(req: HttpRequest) -> impl Responder {
    let allowed_methods = allowed_methods();
    let route = req.path().to_string();
    let mut allowed_methods_header = String::new();

    for method in allowed_methods {
        if method.pattern.is_match(&route) {
            // dbg!(&method);
            allowed_methods_header = method.methods.join(", ");
        };
    }

    dbg!(&route);
    let error_message = format!(
        "This endpoint only supports {} requests",
        allowed_methods_header
    );
    HttpResponse::MethodNotAllowed()
        .append_header(("Allow", allowed_methods_header))
        .json(serde_json::json!({
            "error": "Method not allowed",
            "message": error_message,
        }))
}
