use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use env_logger::{Builder, WriteStyle};
use log::{info, warn};

mod constants;
mod controllers;
mod extractors;
mod models;
mod utils;

use crate::controllers::{my_web_controller::*, static_controller::static_controllers};
use crate::utils::env::{get_cwd, get_log_level, set_env_urls, PageConfiguration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = Builder::new();

    builder
        .filter(None, get_log_level())
        .write_style(WriteStyle::Always)
        .init();

    match get_cwd() {
        Ok(_) => info!("Successfully retrieved current directory"),
        Err(err) => warn!("Error getting current directory: {}", err),
    }

    let PageConfiguration {
        server_address,
        server_port,
        ..
    } = set_env_urls();

    let server_address_conf: String = format!("{server_address}:{server_port}");

    info!("Welcome to Yay_CRM");
    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .configure(index_html_controllers)
            .configure(static_controllers)
    })
    .bind(server_address_conf)
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
