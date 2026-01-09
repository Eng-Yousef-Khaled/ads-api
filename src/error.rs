use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum APIResponseError {
    #[display("عذراً، حدث خطا. يرجى اعادة المحاولة لاحقاً.")]
    InternalError,
    #[display("data not found")]
    NotFound,
    #[display("bad request")]
    BadRequest,
}

impl ResponseError for APIResponseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIResponseError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            APIResponseError::NotFound => StatusCode::NOT_FOUND,
            APIResponseError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}
