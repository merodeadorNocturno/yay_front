use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, web::ServiceConfig, Error, HttpRequest};
use log::{debug, info, warn};

use crate::utils::env::get_cwd;

// use crate::constants::connection::set_environment_variable;

#[get("/{filename:.*}")]
async fn scripts_static(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    match get_cwd() {
        Ok(_) => info!("Successfully retrieved current directory"),
        Err(err) => warn!("Error getting current directory: {}", err),
    }
    // debug!("REQ :: {:?}", &req);
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();

    // let my_path = format!("./{:?}", path);
    debug!("PATH:: {:?}", &path);
    let file = fs::NamedFile::open_async(path).await?;

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

pub fn static_controllers(cfg: &mut ServiceConfig) {
    cfg.service(scripts_static);
}
