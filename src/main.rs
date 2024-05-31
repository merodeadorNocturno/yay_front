use actix_web::{middleware, web::Data, App, HttpServer};
// use actix_cors::Cors;
use actix_web_httpauth::extractors::bearer::Config;
use env_logger::{Builder, WriteStyle};
use log::{info, warn};

mod constants;
mod controllers;
mod error;
mod extractors;
mod middleware_local;
mod models;
mod utils;

use crate::controllers::{my_web_controller::*, static_controller::static_controllers};

use crate::middleware_local::{
    cors::cors, err_handlers::err_handlers, logger::logger, security_headers::security_headers,
};
use crate::models::types;
use crate::utils::env::{get_cwd, get_log_level, set_env_urls, PageConfiguration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = Builder::new();
    let secret = "lIoZjhkjsQYpiU08LFGHaJUrddNP1g51dViYZhUuzKF4an4Qkz9MNfvIjiIT5Ude";

    builder
        .filter(None, get_log_level())
        .write_style(WriteStyle::Always)
        .init();

    match get_cwd() {
        Ok(_) => info!("Successfully retrieved current directory"),
        Err(err) => warn!("Error getting current directory: {}", err),
    }

    let types::Config {
        client_origin_url, ..
    } = types::Config::default();

    let PageConfiguration {
        server_address,
        server_port,
        ..
    } = set_env_urls();

    let server_address_conf: String = format!("{server_address}:{server_port}");

    info!("Welcome to Yay_CRM");
    HttpServer::new(move || {
        // let cors = Cors::permissive().max_age(3600);
        // let auth0_config = Auth0Config::default();
        // let auth0_config = Auth0Config;

        App::new()
            .wrap(cors(&client_origin_url))
            .wrap(err_handlers())
            .wrap(security_headers())
            .wrap(logger())
            // .app_data(Data::new(auth0_config.clone()))
            .app_data(Data::new(secret))
            // .wrap_fn(validate_token_middleware)
            .app_data(Config::default().realm("Restricted Area"))
            .wrap(middleware::NormalizePath::trim())
            .configure(index_html_controllers)
            .configure(static_controllers)
    })
    .bind(server_address_conf)
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
