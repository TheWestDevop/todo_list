use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::todos;
use chrono::NaiveDateTime;



#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub iscompleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="todos"]
pub struct NewTodo {
    pub title: String,
}
impl NewTodo {
   pub fn new(title: String) -> NewTodo {
        NewTodo {
            title
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoData {
    pub title: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct UpdateTodoData {
    pub title: String,
    pub iscompleted: bool,
}
