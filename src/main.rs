use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

use crate::utils::app_state::AppState;

mod error;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), error::ServiceError> {
    println!("Hello, world!");
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = (utils::constants::PORT).clone();
    let address = (utils::constants::ADDRESS).clone();
    let database_url = (utils::constants::DATABASE_URL).clone();

    println!("port: {}", port);

    let db: DatabaseConnection = Database::connect(database_url).await.map_err(|err| {
        error::ServiceError::DBConnectionError {
            error_message: err.to_string(),
        }
    })?;

    println!("Starting server on {}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::user_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::block_routes::config)
            .configure(routes::tx_routes::config)
    })
    .bind((address, port))
    .map_err(|err| error::ServiceError::BindAddressError {
        error_message: err.to_string(),
    })?
    .run()
    .await
    .map_err(|err| error::ServiceError::RunServerError {
        error_message: err.to_string(),
    })
}
