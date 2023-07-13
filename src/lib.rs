use crate::api::{add_post};
use crate::view::index;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use sqlx::SqlitePool;
use std::net::TcpListener;
use actix_files as fs;

mod api;
mod view;

/// Creates an HTTP server with the given listener and sets the database pool in the app state.
/// All endpoints from this server start at /api
pub fn run_server(listener: TcpListener, db_pool: SqlitePool) -> Result<Server, std::io::Error> {
    let addr = listener.local_addr().unwrap();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();



    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(handlebars.clone()))
            .service(index)
            .service(
                web::scope("/api")
                    .wrap(cors)
                    .service(add_post)
            )
            .service(fs::Files::new("/css", "public/css"))
    })
    .listen(listener)?
    .run();

    println!("Server is running at {}", addr);

    Ok(server)
}

/// Connects to the database based on the given connection_string and returns out the pool
pub async fn setup_database(connection_string: String) -> SqlitePool {
    let pool = SqlitePool::connect(&connection_string)
        .await
        .unwrap_or_else(|err| {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        });

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Valid migrations should exist in migrations/");

    pool
}
