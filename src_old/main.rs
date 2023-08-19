use actix_web::{guard, web, App, HttpServer, middleware};
mod routes;
mod handler;
mod models;
pub mod db_models;
pub mod schema;


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
