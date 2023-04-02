use std::env;

use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

mod auth;
mod crypto;
mod db_ops;
mod dog;
mod pw;
mod routes;
mod session;
mod util;

async fn create_pg_pool() -> sqlx::Pool<sqlx::Postgres> {
    let pg_usr =
        &env::var("POSTGRES_USER").expect("postgres user to be defined in environment")[..];
    let pg_pw =
        &env::var("POSTGRES_PASSWORD").expect("postgres password to be defined in environment")[..];
    let pg_db =
        &env::var("POSTGRES_DB").expect("postgres db name to be defined in environment")[..];
    let db_url = &format!("postgres://{}:{}@localhost:5432/{}", pg_usr, pg_pw, pg_db)[..];

    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("pool to startup")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = create_pg_pool().await;

    let srv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("content-type", "application/json")))
            .app_data(web::Data::new(pool.to_owned()))
            .service(
                web::scope("/api/v1")
                    .service(routes::index)
                    .service(routes::login)
                    .service(routes::register)
                    .service(routes::profile),
            )
    });

    let srv = srv.bind(("0.0.0.0", 8000)).expect("server binds");
    println!("Server listening on port 8000");

    srv.run().await
}
