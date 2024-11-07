use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};
use std::{env, io};

use crate::constants::connection::set_environment_variable;

pub fn get_cwd() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    info!("Current working directory: {}", current_dir.display());

    Ok(())
}

fn get_server_conf() -> String {
    let server_address = set_environment_variable("SERVER_ADDRESS", "0.0.0.0");
    let server_port = set_environment_variable("SERVER_PORT", "8081");
    let server_protocol = set_environment_variable("SERVER_PROTOCOL", "http");

    format!("{server_protocol}://{server_address}:{server_port}/")
}

fn get_backend_url() -> String {
    let backend_address = set_environment_variable("BACKEND_ADDRESS", "0.0.0.0");
    let mut backend_port = set_environment_variable("BACKEND_PORT", "8080");
    let backend_protocol = set_environment_variable("BACKEND_PROTOCOL", "http");

    if backend_port != "80".to_string() {
        backend_port = format!(":{}", &backend_port);
    } else {
        backend_port = "".to_string();
    }
    format!("{backend_protocol}://{backend_address}{backend_port}/")
}

pub fn get_log_level() -> LevelFilter {
    let log_level = set_environment_variable("RUST_LOG", "debug");

    let level_filter = match log_level.as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Debug,
    };

    level_filter
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageConfiguration {
    pub server_conf: String,
    pub backend_url: String,
    pub server_address: String,
    pub server_port: String,
    pub server_protocol: String,
    pub title: String,
}

pub fn set_env_urls() -> PageConfiguration {
    PageConfiguration {
        server_conf: get_server_conf(),
        backend_url: get_backend_url(),
        server_address: set_environment_variable("SERVER_ADDRESS", "0.0.0.0"),
        server_port: set_environment_variable("SERVER_PORT", "8081"),
        server_protocol: set_environment_variable("SERVER_PROTOCOL", "http"),
        title: set_environment_variable("PAGE_TITLE", "CRM"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfiguration {
    pub auth0_audience: String,
    pub auth0_domain: String,
}

pub fn set_auth_urls() -> AuthConfiguration {
    AuthConfiguration {
        auth0_audience: set_environment_variable("AUTH0_AUDIENCE", "https://yay.local"),
        auth0_domain: set_environment_variable("AUTH0_DOMAIN", "http://some_domain.local"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

pub fn set_client_credentials() -> ClientCredentials {
    ClientCredentials {
        client_id: set_environment_variable("CLIENT_ID", "1234556"),
        client_secret: set_environment_variable("CLIENT_SECRET", "shhhh"),
    }
}
