use actix_web::{
    error,
    http::{header, StatusCode},
    HttpResponse, HttpResponseBuilder,
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum MCError {
    #[error("Invalid Request")]
    InvalidRequest,
    #[error("Server Offline")]
    ServerOffline,
}

impl error::ResponseError for MCError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            MCError::InvalidRequest => StatusCode::BAD_REQUEST,
            MCError::ServerOffline => StatusCode::REQUEST_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType(mime::TEXT_PLAIN))
            .body(self.to_string())
    }
}
