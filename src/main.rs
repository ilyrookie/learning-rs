use actix_web::{HttpServer, App, web};
use dotenv;
use std::env;

#[macro_use]
extern crate diesel;

pub mod models;
pub mod controllers;
pub mod database;
pub mod schema;
pub mod middleware;
pub mod misc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    database::connect(&db_url);
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/v1")
                .configure(controllers::users)
            )
            .configure(controllers::users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}