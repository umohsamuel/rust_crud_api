use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
// use std::env;
use std::sync::Mutex;

mod db;
mod handlers;
mod jwt;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db = db::Database::new().expect("Failed to initialize database");

    let jwt_secret_env = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    db.set_setting("JWT_SECRET", &jwt_secret_env)
        .expect("Failed to set JWT secret in DB");

    let db_data = web::Data::new(Mutex::new(db));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE,
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
            ])
            .expose_headers(vec!["Set-Cookie"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .service(handlers::register)
            .service(handlers::login)
            .service(handlers::refresh)
            .service(
                web::scope("/api")
                    .wrap(handlers::AuthMiddleware {
                        jwt_secret: jwt_secret_env.clone(),
                    })
                    .service(handlers::get_tasks)
                    .service(handlers::create_task)
                    .service(handlers::update_task)
                    .service(handlers::delete_task),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
