use std::io;

use actix_web::{middleware, App, HttpServer};
use dotenvy::{dotenv, Error as DotenvError};

use backend::{di::AppDependencies, entities::app_config::AppConfig, http, routes};

#[actix_web::main]
async fn main() -> io::Result<()> {
    if let Err(error) = dotenv() {
        if !matches!(error, DotenvError::Io(ref source) if source.kind() == io::ErrorKind::NotFound)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("failed to load .env: {error}"),
            ));
        }
    }

    let config = AppConfig::from_env()
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    let dependencies = AppDependencies::build(config.clone())
        .await
        .map_err(|error| io::Error::new(io::ErrorKind::ConnectionRefused, error.to_string()))?;
    let bind_address = config.host.clone();
    let port = config.port;

    println!(
        "Launching {} on http://{bind_address}:{port}",
        config.app_name
    );

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(http::cors(&config.cors_allowed_origin))
            .app_data(http::json_config())
            .configure(|service_config| dependencies.configure(service_config))
            .configure(routes::configure)
            .default_service(actix_web::web::route().to(routes::not_found))
    })
    .bind((bind_address, port))?
    .run()
    .await
}
