use crate::models::todo::Todo;
use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse,  dev::HttpResponseBuilder, error, http::StatusCode};
use derive_more::{Display, Error};

#[derive(Serialize, Deserialize)]
pub struct TodosResponse {
    pub status: bool,
    pub data: Option<Vec<Todo>>,
    pub message: Option<String>,
}
impl TodosResponse {
    pub fn success(status: bool, data:Option<Vec<Todo>>,message: Option<String>) -> HttpResponse { 
       HttpResponse::Ok().json(
            TodosResponse {
                status,
                data,
                message,
           }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoResponse {
    pub status: bool,
    pub data: Option<Todo>,
    pub message: Option<String>,
}
impl TodoResponse {
    pub fn success(status: bool, data:Option<Todo>,message: Option<String>) -> HttpResponse {
        HttpResponse::Ok().json(
            TodoResponse {
                status,
                data,
                message,
           }
        )
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorMessage { 
    pub status: bool, 
    pub status_code: u16, 
    pub message: String 
} impl ErrorMessage {
    fn message(status: bool, status_code: u16, message: String) -> ErrorMessage {
        ErrorMessage {
            status,
            status_code,
            message
          }
    }
}
#[derive(Debug, Display, Error)]
pub enum TodoError {
    // #[display(fmt = "Validation error on field: {}", field)]
    // ValidationError { field: String },

    #[display(fmt = "Something went wrong, Please try again later.")]
    InternalError,

    #[display(fmt = "You sent a bad request")]
    BadClientData ,

    #[display(fmt = "url not found, check your input again")]
    NotFound,

    // #[display(fmt = "auth")]
    // TokenNotFound,

}

impl error::ResponseError for TodoError {
     fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(ErrorMessage::message(false, self.status_code().as_u16(), self.to_string())
        )
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            // TodoError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            TodoError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            TodoError::BadClientData => StatusCode::BAD_REQUEST,
            TodoError::NotFound => StatusCode::NOT_FOUND,
            // TodoError::TokenNotFound => StatusCode::NON_AUTHORITATIVE_INFORMATION
        }
    }
}