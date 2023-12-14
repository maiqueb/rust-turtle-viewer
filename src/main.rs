extern crate diesel;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod db;
mod error_handler;
mod turtles;

mod schema;
#[cfg(test)]
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_user = env::var("DB_USER").expect("Please set 'DB_USER' in .env");
    let db_password = env::var("DB_PASSWORD").expect("variable 'DB_PASSWORD' must be set");
    let database = env::var("DB_NAME").expect("variable 'DB_NAME' must be set");
    let db_ip = env::var("DB_IP").expect("variable 'DB_IP' must be set");

    let port: u16 = env::var("DB_PORT")
        .map(|port| port.parse::<u16>().unwrap())
        .unwrap_or(5432);

    env::set_var(
        "DATABASE_URL",
        format!("postgresql://{db_user}:{db_password}@{db_ip}:{port}/{database}"),
    );
    db::init();

    let host = env::var("HOST").expect("Please set host in .env");
    let port = env::var("PORT").expect("Please set port in .env");

    HttpServer::new(|| App::new().configure(turtles::init_routes))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
