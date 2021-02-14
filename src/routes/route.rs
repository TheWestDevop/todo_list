use actix_web::{ get, post, put, patch, delete, web::{Data, Json, Path}, HttpResponse, Result};
use crate::utils::response::{ TodosResponse, TodoResponse, TodoError};
use crate::models::todo::{ TodoData, UpdateTodoData};
use crate::models::state::{ AppState };
use crate::actors::todo_actors::{GetAllTodos ,Create, Update, Delete, Completed};





#[get("")]
pub async fn get_todos(state: Data<AppState>) -> Result<HttpResponse, TodoError> {
    let db = state.as_ref().db.clone();
    match db.send(GetAllTodos).await {
        Ok(Ok(data)) => Ok(TodosResponse::success(true, Some(data), Some("All Todos".to_string()))),
        _ => Err(TodoError::InternalError)
    }
}
#[post("/new")]
pub async fn create_todo(user_todo: Json<TodoData>, state: Data<AppState>) -> Result<HttpResponse, TodoError> {
    let db = state.as_ref().db.clone();
    let new_todo = user_todo.into_inner();
    
    if new_todo.title.is_empty() {
        return Err(TodoError::BadClientData)
    }


    match db.send(Create::this(new_todo.title.to_string())).await {
        Ok(Ok(data)) => Ok(TodoResponse::success(true, Some(data), Some("Todo Added Successfully".to_string()))),
        _ => Err(TodoError::InternalError)
    }
}

#[put("/update/{tid}")]
pub async fn update_todo(Path(todo_id): Path<i32>, user_todo: Json<UpdateTodoData>, state: Data<AppState>) -> Result<HttpResponse, TodoError> {
    let db = state.as_ref().db.clone();
    let new_todo = user_todo.into_inner();

    match db.send(Update::this(todo_id,new_todo.title.to_string(),new_todo.iscompleted)).await {
        Ok(Ok(data)) => Ok(TodoResponse::success(true, Some(data), Some("Todo Updated Successfully".to_string()))),
        _ => Err(TodoError::InternalError)
    }
}

#[patch("/{tid}/completed")]
pub async fn todo_completed(Path(todo_id): Path<i32>, state: Data<AppState>) -> Result<HttpResponse, TodoError> {
    let db = state.as_ref().db.clone();

    match db.send(Completed::this(todo_id)).await {
        Ok(Ok(data)) => Ok(TodoResponse::success(true, Some(data), Some("Todo Completed Successfully".to_string()))),
        _ => Err(TodoError::InternalError)
    }
}

#[delete("/remove/{tid}")]
pub async fn delete_todo(Path(todo_id): Path<i32>, state: Data<AppState>) -> Result<HttpResponse, TodoError> {
    let db = state.as_ref().db.clone();
    match db.send(Delete::this(todo_id)).await {
        Ok(Ok(_)) => Ok(TodoResponse::success(true, None, Some("Todo Deleted Successfully".to_string()))),
        _ => Err(TodoError::InternalError)
    }
}
