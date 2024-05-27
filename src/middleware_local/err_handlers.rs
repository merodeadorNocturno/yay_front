use actix_web::{
    dev::ServiceResponse,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    HttpResponse, Result,
};

use crate::models::types::ErrorMessage;

pub fn err_handlers<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error)
        .handler(StatusCode::NOT_FOUND, not_found)
}

pub fn internal_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let http_res = HttpResponse::InternalServerError()
        .content_type(ContentType::json())
        .body(
            serde_json::to_string(&ErrorMessage {
                error: None,
                error_description: None,
                message: "Internal server error".to_string(),
            })
            .unwrap(),
        );

    Ok(ErrorHandlerResponse::Response(
        res.into_response(http_res.map_into_right_body()),
    ))
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let http_res = HttpResponse::NotFound()
        .content_type(ContentType::json())
        .body(
            serde_json::to_string(&ErrorMessage {
                error: None,
                error_description: None,
                message: "Not Found".to_string(),
            })
            .unwrap(),
        );

    Ok(ErrorHandlerResponse::Response(
        res.into_response(http_res.map_into_right_body()),
    ))
}
