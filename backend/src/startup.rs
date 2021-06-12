use std::sync::Arc;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{http, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::config::Settings;
use crate::twitter::routes::pull::{backfill, pull};
use crate::twitter::routes::serve::{hello, health, tweets4};

#[tracing::instrument(skip(pg_pool, config))]
pub fn run(addr: &str, pg_pool: PgPool, config: Settings) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pg_pool); //important - else get https://stackoverflow.com/questions/56117273/actix-web-reports-app-data-is-not-configured-when-processing-a-file-upload
    let arc_config = web::Data::new(Arc::new(config));

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
            .app_data(arc_config.clone())
    })
    .bind(addr)?
    .run();
    Ok(server) //refactored to return a server so that we can use it in tokio::spawn in tests
}
