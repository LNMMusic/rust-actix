use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};


// USING AN ENUM TO GROUP ERRORS and USE IT AS DATATYPE ON RESULT
// [LIKE A TRAIT]
#[derive(Debug, Display, Error)]
pub enum HttpErrors {
    #[display(fmt = "Error: {}", Message.message)]
    StandardError { message: String },
}
impl error::ResponseError for HttpErrors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            HttpErrors::StandardError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}