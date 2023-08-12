use actix_web::{guard, web, App, HttpServer, middleware};
use sqlx::{sqlite::*, Connection, Error, Row, Sqlite};
mod routes;
mod models;
mod handler;

async fn prefill_db_with_tables(sqlitefile: &str) {
    // Connect to the sqlite file from the string and then execute sql from the file.sql
    let mut conn = SqliteConnection::connect(sqlitefile).await.unwrap();
    let sql = include_str!("../migrations/create_tables.sql");
    sqlx::query(sql).execute(&mut conn).await.unwrap();
    dbg!("Tables created");
    let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    for row in tables {
        let name: String = row.get(0);
        dbg!(name);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // prefill_db_with_tables("database.db").await;
    HttpServer::new(move || {
        App::new()
            .app_data("Hellooooooooo")
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/")
                            .route(web::get().to(routes::index))
                            .default_service(
                                web::route()
                                    .guard(guard::Not(guard::Get()))
                                    .to(routes::method_not_allowed_handler),
                            ),
                    )
                    .service(
                        web::resource("/user/{userid}")
                            .route(web::get().to(routes::get_user))
                            .default_service(
                                web::route()
                                    .guard(guard::Not(guard::Get()))
                                    .to(routes::method_not_allowed_handler),
                            ),
                    ),
            )
    })
    .bind("localhost:8080")?
    .run()
    .await
}
