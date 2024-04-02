
use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum  Error {
    LoginFail,
    NotFound,
    AuthFailed,
    InvalidToken,
}

impl  IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        print!("---> {:<12} - {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}