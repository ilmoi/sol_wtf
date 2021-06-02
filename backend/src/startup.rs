use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{http, App, HttpServer};

use crate::routes::all_routes::hello;
use crate::routes::all_routes::tweets4;
use sqlx::PgPool;

pub fn run(addr: &str, pg_pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(hello)
            .service(tweets4)
            .app_data(pg_pool.clone())
    })
    .bind(addr)?
    .run();
    Ok(server) //refactored to return a server so that we can use it in tokio::spawn in tests
}
