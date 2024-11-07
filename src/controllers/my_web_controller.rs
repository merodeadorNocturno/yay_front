use actix_session::Session;
use actix_web::{
    // cookie::Cookie,
    error::ResponseError,
    get,
    http::{
        header::{ContentType, LOCATION},
        StatusCode,
    },
    // post,
    web::{self, Query, ServiceConfig},
    HttpResponse,
    // HttpServer,
    Responder,
};
use actix_web_openidconnect::openid_middleware::Authenticated;
use derive_more::Display;
use handlebars::{Handlebars, RenderError};
// use jsonwebtoken::{decode, decode_header, Header};
use log::{debug, error};
use oauth2::reqwest::async_http_client;
use openidconnect::{
    core::{
        CoreClient, CoreIdTokenVerifier, CoreJwsSigningAlgorithm, CoreProviderMetadata,
        CoreResponseType, CoreSubjectIdentifierType,
    },
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyAdditionalProviderMetadata,
    IssuerUrl, JsonWebKeySetUrl, Nonce, RedirectUrl, ResponseTypes,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

use crate::utils::{
    env::{
        set_auth_urls, set_client_credentials, set_env_urls, AuthConfiguration, ClientCredentials,
    },
    fs_utils::{read_hbs_template, register_templates},
};

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

async fn index_html(auth_data: Authenticated) -> Result<String, RenderError> {
    error!("{:?}", &auth_data.access);

    // let key = b"lIoZjhkjsQYpiU08LFGHaJUrddNP1g51dViYZhUuzKF4an4Qkz9MNfvIjiIT5Ude";
    // validate();
    // validate(key, auth.token().to_string());
    let mut handlebars = Handlebars::new();
    // let tkn = format!("@@@@@@@@@@@@@@@@@@@@@@@@ {:?}", auth);
    // let tkn = format!("@@@@@@@@@@@@@@@@@@@@@@@@ {}", &auth.token());
    // warn!("template path: {}", tkn);

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

async fn login() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();

    let this_path = Path::new("./src/static");
    register_templates(this_path, &mut handlebars);

    let login_hbs = "login";

    let index = match read_hbs_template(&login_hbs) {
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

#[derive(Debug, Deserialize, Serialize)]
struct QueryStruct {
    code: String,
    state: String,
}

#[derive(Debug, Display)]
pub enum CallbackError {
    CallbackNotOk = 0,
    CallbackFailure = 1,
}

impl ResponseError for CallbackError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CallbackError::CallbackNotOk => StatusCode::NOT_FOUND,
            CallbackError::CallbackFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CallbackErrorJson {
    pub error: String,
}

#[get("/callback_1")]
async fn callback_n(
    session: Session,
    params: Query<QueryStruct>,
) -> Result<HttpResponse, CallbackError> {
    let p = params.into_inner();

    let this_code = p.code;
    let this_state = p.state;

    let code = AuthorizationCode::new(this_code);
    let state = CsrfToken::new(this_state);
    let session_state = match session.get::<CsrfToken>("csrf_state") {
        Ok(this_session) => match this_session {
            Some(this_session_state) => this_session_state,
            None => {
                error!(
                    "{:?} -- {:?} ** {:?} ## {:?}",
                    &code,
                    &state,
                    &this_session,
                    &session.entries()
                );
                return Ok(HttpResponse::NotFound().json(CallbackErrorJson {
                    error: format!("{}", CallbackError::CallbackNotOk),
                }));
            }
        },
        Err(e) => {
            return Ok(HttpResponse::NotFound().json(CallbackErrorJson {
                error: format!("{e:?} -- {}", CallbackError::CallbackFailure),
            }));
        }
    };

    if state.secret() != session_state.secret() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let response_types =
        ResponseTypes::new(vec![CoreResponseType::Code, CoreResponseType::IdToken]);

    let subject_identifier_types = vec![
        CoreSubjectIdentifierType::Public,
        CoreSubjectIdentifierType::Pairwise,
    ];

    let provider_metadata = EmptyAdditionalProviderMetadata::default();

    let AuthConfiguration { auth0_domain, .. } = set_auth_urls();

    let core_client_url = format!("https://{auth0_domain}/");

    let client = CoreClient::from_provider_metadata(
        CoreProviderMetadata::new(
            IssuerUrl::new("{&core_client_url}.well-known/openid-configuration".to_string())
                .unwrap(),
            AuthUrl::new("{&core_client_url}/authorize".to_string()).unwrap(),
            JsonWebKeySetUrl::new("{&core_client_url}.well-known/jwks.json".to_string()).unwrap(),
            vec![response_types],
            subject_identifier_types,
            vec![
                CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
                CoreJwsSigningAlgorithm::RsaSsaPssSha256,
            ],
            provider_metadata,
        ),
        ClientId::new("iGrVG2EzdK3W4J6CUT5N8fnqYhdrRrmt".to_string()),
        Some(ClientSecret::new(
            "0vAld54MISZHU5uI_XjzuKP3_wYwWsoM_KlNlkSKb3SBTa3SY08-HuP6xi6e6nzL".to_string(),
        )),
    )
    .set_redirect_uri(RedirectUrl::new("http://node.local/".to_string()).unwrap());

    let token_response = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .unwrap();

    let id_token_verifier: CoreIdTokenVerifier = client.id_token_verifier();
    let nonce = session.get::<Nonce>("nonce").unwrap().unwrap();

    let id_token_claims = token_response
        .extra_fields()
        .id_token()
        .expect("SERVER DID NOT RETURN ID TOKEN")
        .claims(&id_token_verifier, &nonce);

    let user_email = match id_token_claims {
        Ok(claims) => claims.email().map(|email| email.to_string()),
        Err(_) => None,
    };

    match user_email {
        Some(email) => match session.insert("user_email", email) {
            Ok(_) => (),
            Err(_) => (),
        },
        None => (),
    };
    // session.insert("user_email", user_email);

    Ok(HttpResponse::Found()
        .append_header((LOCATION, "/"))
        .finish())
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    sub: String,
    name: String,
    email: String,
    // Add other fields as necessary
}

#[get("/callback_2")]
async fn callback(
    auth_request: web::Query<AuthRequest>,
    client: web::Data<Client>,
) -> impl Responder {
    error!("{:?}, {:?}", &auth_request, &client);

    let AuthConfiguration { auth0_domain, .. } = set_auth_urls();

    let ClientCredentials {
        client_id,
        client_secret,
    } = set_client_credentials();

    let redirect_uri = "https://node.local/callback".to_string();
    let token_url = format!("https://{}/oauth/token", auth0_domain);

    let code = &auth_request.code;
    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("code", code),
        ("redirect_uri", &redirect_uri),
    ];
    error!("ALL VARS SET, {:?}", params);
    let token_response: TokenResponse = client
        .post(&token_url)
        .form(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    error!("SHOULD'VE GOTTEN TOKEN RESPONSE, {:?}", params);

    let user_info_url = format!("https://{}/userinfo", &auth0_domain);

    let user_info: UserInfo = client
        .get(&user_info_url)
        .bearer_auth(&token_response.access_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    error!("SHOULD'VE GOTTEN USER INFO, {:?}", params);

    HttpResponse::Ok().json(user_info)
}

// #[get("callback")]
async fn callback_3(
    auth_request: web::Query<AuthRequest>,
    session: Session,
    client: web::Data<Client>,
) -> impl Responder {
    // let state = session.get::<String>("state").unwrap_or(None);
    let state = Some(auth_request.state.clone().to_string());
    error!("REQUEST:: {:?}", &auth_request);
    if state.is_none() || state.unwrap() != auth_request.state {
        return HttpResponse::BadRequest().body("Invalid state parameter.");
    }

    let code = &auth_request.code;
    let AuthConfiguration { auth0_domain, .. } = set_auth_urls();

    let ClientCredentials {
        client_id,
        client_secret,
    } = set_client_credentials();

    let redirect_uri = "https://node.local/callback".to_string();

    let token_url = format!("https://{}/oauth/token", &auth0_domain);

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("code", code),
        ("redirect_uri", &redirect_uri),
    ];

    let response = client.post(&token_url).form(&params).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    println!("Response from Auth0: {}", response_text);

    let token_response: Result<TokenResponse, serde_json::Error> =
        serde_json::from_str(&response_text);

    match token_response {
        Ok(tokens) => {
            let user_info_url = format!("https://{}/userinfo", &auth0_domain);
            let user_info: UserInfo = client
                .get(&user_info_url)
                .bearer_auth(&tokens.access_token)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            session.insert("access_token", tokens.access_token).unwrap();
            session.insert("profile", &user_info).unwrap();
            session.renew(); // Secure the session by renewing its identifier

            HttpResponse::TemporaryRedirect()
                .append_header(("Location", "/user"))
                .finish()
        }
        Err(e) => {
            println!("Failed to deserialize token response: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to authenticate")
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Claims {
    nonce: String,
    aud: String,
    exp: usize,
    iss: String,
}

#[derive(Deserialize)]
struct OidcCallbackData {
    id_token: String,
    state: String,
    code: String,
    access_token: String,
}

#[derive(Deserialize, Serialize)]
struct OidcProviderInfo {
    issuer: String,
    client_id: String,
    client_secret: String,
    // Add other fields as needed (e.g., authorization_endpoint, token_endpoint)
}

// #[post("/callback")]
// async fn callback(session: Session, form: web::Form<OidcCallbackData>) -> impl Responder {
//     let nonce = session.get::<String>("nonce");

//     HttpResponse::Ok()
// }

#[get("/user")]
async fn user(session: Session) -> impl Responder {
    if let Some(profile) = session.get::<UserInfo>("profile").unwrap() {
        HttpResponse::Ok().json(profile)
    } else {
        HttpResponse::Unauthorized().body("Not logged in")
    }
}

pub fn index_html_controllers(cfg: &mut ServiceConfig) {
    cfg.route(
      "/",
      // get().to(|| async move {
      web::get().to(|auth_data: Authenticated| async move {
        let yay_help_template = index_html(auth_data).await;
        // let yay_help_template = index_html().await;

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
      "/login",
      web::get().to(|| async move {
      // get().to(|auth_data: Authenticated| async move {
        // let yay_help_template = login(auth_data).await;
        let yay_login_template = login().await;

        match yay_login_template {
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

    cfg.service(callback);
    cfg.service(user);

    cfg.route(
    "/help_enterprise",
    web::get().to(|| async move {
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
    web::get().to(|| async move {
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
    web::get().to(|| async move {
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
