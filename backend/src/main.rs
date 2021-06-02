use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, App, HttpServer};
use env_logger::Env;

use backend::config::get_config;
use backend::routes::all_routes::hello;
use backend::routes::all_routes::tweets4;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("failed to read settings");

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
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
    })
    .bind(format!("127.0.0.1:{}", config.app_port))?
    .run()
    .await
}
