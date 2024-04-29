use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};
use serde_json::json;
use std::path::Path;

use crate::utils::{
    env::set_env_urls,
    fs_utils::{read_hbs_template, register_templates},
};
use handlebars::{Handlebars, RenderError};
use log::{debug, error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Title {
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TitleError {
    pub error: String,
}

impl TitleError {
    pub fn new(error: String) -> TitleError {
        TitleError { error }
    }
}

async fn index_html() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();

    let this_path = Path::new("./src/static");
    register_templates(this_path, &mut handlebars);

    let index_hbs = "index";

    let index = match read_hbs_template(&index_hbs) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit title:: {}",
                e.to_string()
            );
            TitleError::new(e.to_string()).error
        }
    };

    let default_env = set_env_urls();

    let index_template = handlebars.render_template(&index, &json!(default_env))?;
    Ok(index_template)
}

async fn help_enterprise() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();

    let template_path = "help_enterprise_html";

    let title_data = Title {
        title: "".to_string(),
    };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit help_enterprise:: {}",
                e.to_string()
            );
            TitleError::new(e.to_string()).error
        }
    };

    let yay_help = handlebars.render_template(&template_contents, &json!(title_data))?;
    Ok(yay_help)
}

async fn enterprise_avance_panel() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();

    let template_path = "help_enterprise_avance_html";

    let help_data = Title {
        title: "".to_string(),
    };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit help_enterprise:: {}",
                e.to_string()
            );
            TitleError::new(e.to_string()).error
        }
    };

    let yay_help = handlebars.render_template(&template_contents, &help_data)?;
    Ok(yay_help)
}

async fn enterprise_services_panel() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();

    let template_path = "help_enterprise_services_html";
    debug!("template path: {}", template_path);
    let help_data = Title {
        title: "".to_string(),
    };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit help_enterprise:: {}",
                e.to_string()
            );
            TitleError::new(e.to_string()).error
        }
    };

    let yay_help = handlebars.render_template(&template_contents, &help_data)?;
    Ok(yay_help)
}

pub fn index_html_controllers(cfg: &mut ServiceConfig) {
    cfg.route(
    "/",
    get().to(|| async move {
      let yay_help_template = index_html().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
          .append_header(("HX-Trigger", "help_table"))
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load title: {}</span>",
            e.to_string())
          )
      }
    })
  );

    cfg.route(
    "/help_enterprise",
    get().to(|| async move {
      let yay_help_template = help_enterprise().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load title: {}</span>",
            e.to_string())
          )
      }
    })
  );

    cfg.route(
    "/help_enterprise_avance",
    get().to(|| async move {
      let yay_help_template = enterprise_avance_panel().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load title: {}</span>",
            e.to_string())
          )
      }
    })
  );

    cfg.route(
    "/help_enterprise_servicios",
    get().to(|| async move {
      let yay_help_template = enterprise_services_panel().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load title: {}</span>",
            e.to_string())
          )
      }
    })
  );
}
