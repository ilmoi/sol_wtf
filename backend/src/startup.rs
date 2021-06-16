use std::sync::Arc;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{http, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::config::Settings;
use crate::twitter::routes::pull::{backfill, pull};
use crate::twitter::routes::serve::{health, hello, tweets4};

#[tracing::instrument(skip(pool, config))]
pub fn run_server(
    addr: &str,
    pool: Arc<PgPool>,
    config: Arc<Settings>,
) -> Result<Server, std::io::Error> {
    //important to add web::Data() - else get https://stackoverflow.com/questions/56117273/actix-web-reports-app-data-is-not-configured-when-processing-a-file-upload
    let pool = web::Data::new(pool);
    let config = web::Data::new(config);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default()) //add request_id to actix events
            .service(hello)
            .service(health)
            .service(tweets4)
            .service(pull)
            .service(backfill)
            .app_data(pool.clone())
            .app_data(config.clone())
    })
    .bind(addr)?
    .run();
    Ok(server) //refactored to return a server so that we can use it in tokio::spawn in tests
}
