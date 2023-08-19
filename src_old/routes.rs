// use crate::models::{AllowedMethods};
// use crate::db_models::{User, Userauth, Useranalytic, Tweet, Reply, Like, Follower};
use actix_web::{HttpRequest, HttpResponse, Responder, Result, Error};
use chrono::Utc;
use regex::Regex;

#[derive(Debug)]
pub struct AllowedMethods {
    pub pattern: Regex,
    pub methods: Vec<String>,
}


impl AllowedMethods {
    pub fn new(regex: String, methods: Vec<String>) -> Self {
        Self {
            pattern: Regex::new(&regex).unwrap(),
            methods,
        }
    }
}



pub async fn index() -> Result<HttpResponse, Error> {
    let start = std::time::Instant::now();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "heartbeat": "OK",
        // Have the server return the time taken for the request to be processed
        "time_taken": format!("{}μs", start.elapsed().as_micros()),
    })))
}


pub async fn get_user(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
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
