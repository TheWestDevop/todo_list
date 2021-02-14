use actix_web::{HttpResponse, Result, get};
use tracing::instrument;
use crate::utils::response::{ TodoResponse, TodoError };

// #[get("")]
// #[instrument]
pub async fn route_not_found() -> Result<HttpResponse, TodoError> {
   Err(TodoError::NotFound)
}



#[get("/health")]
#[instrument]
pub async fn health() -> HttpResponse {
    TodoResponse::success(true,None,Some("I'm Good".to_string()))
}

