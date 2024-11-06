use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, dev::ServiceRequest, middleware, web::Data, App, HttpServer};
use actix_web_httpauth::extractors::bearer::Config;
use actix_web_openidconnect::ActixWebOpenId;
use awc::cookie;
use env_logger::{Builder, WriteStyle};
use log::{error, info, warn};
use reqwest::Client;
use time::Duration;

mod constants;
mod controllers;
mod error;
mod extractors;
mod middleware_local;
mod models;
mod rustlings;
mod utils;

use crate::controllers::{my_web_controller::*, static_controller::static_controllers};

use crate::middleware_local::{
    cors::cors, err_handlers::err_handlers, logger::logger, security_headers::security_headers,
};
use crate::models::types;
use crate::utils::env::{get_cwd, get_log_level, set_env_urls, PageConfiguration};

fn set_cookies_and_session() -> SessionMiddleware<CookieSessionStore> {
    let secret_str =
        "excwCVYdZATlV2YffXoETuzyvx0rpum0BNlSRaapPsGrjUrbiSHsBil7QfV0r0Gzi918xCBySUs9gYa5EEDGTA==";
    let secret_key = cookie::Key::from(secret_str.as_bytes());

    SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
        .cookie_http_only(false)
        .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(1)))
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let should_auth = |req: &ServiceRequest| {
        error!(">>>>>>>>>>>>>>>>>>>>>>>>>>>> {:?}", req);
        req.path().starts_with("/home") && req.method() != actix_web::http::Method::OPTIONS
    };

    let openid = ActixWebOpenId::init(
        "iGrVG2EzdK3W4J6CUT5N8fnqYhdrRrmt".to_string(),
        "e1133f90910f0f896df4861e707dd8eafd16649ddb3a36f95bf97ee4ec575b20".to_string(),
        "https://node.local/callback".to_string(),
        "https://dev-zv75zriia3jcgnej.us.auth0.com/".to_string(),
        should_auth,
        Some("https://node.local/login".to_string()),
        vec!["openid".to_string()],
    )
    .await;

    let mut builder = Builder::new();

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
        let secret = Key::generate();
        let auth_middleware_factory = openid.get_middleware();
        // error!("{:?}", &auth_middleware_factory.);

        App::new()
            .app_data(Data::new(Client::new()))
            .wrap(auth_middleware_factory)
            .wrap(set_cookies_and_session())
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
