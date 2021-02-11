use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::models::todo::{NewTodo, Todo};
use crate::schema::todos::dsl::{id, iscompleted, title, todos};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct Create {
    pub title: String,
}
impl Create {
    pub fn this(todo_title: String) -> Create {
        Create { title: todo_title }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct Update {
    pub id: i32,
    pub title: String,
    pub iscompleted: bool,
}
impl Update {
    pub fn this(todo_id: i32, todo_title: String, todo_iscompleted: bool) -> Update {
        Update {
            id : todo_id,
            title: todo_title,
            iscompleted: todo_iscompleted,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct Delete {
    pub id: i32,
}
impl Delete {
   pub  fn this(todo_id:i32) -> Delete  {
        Delete{
            id:todo_id
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct Completed {
    pub id: i32,
}impl Completed {
    pub  fn this(todo_id:i32) -> Completed  {
         Completed{
             id:todo_id
         }
     }
 }

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Todo>>")]
pub struct GetAllTodos;

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Handler<Create> for DbActor {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        let new_todo = NewTodo::new(msg.title);

        diesel::insert_into(todos)
            .values(new_todo)
            .get_result::<Todo>(&conn)
    }
}

impl Handler<Update> for DbActor {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");

        diesel::update(todos)
            .filter(id.eq(msg.id))
            .set((title.eq(msg.title), iscompleted.eq(msg.iscompleted)))
            .get_result::<Todo>(&conn)
    }
}

impl Handler<Delete> for DbActor {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");

        diesel::delete(todos)
            .filter(id.eq(msg.id))
            .get_result::<Todo>(&conn)
    }
}

impl Handler<Completed> for DbActor {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, msg: Completed, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        diesel::update(todos)
            .filter(id.eq(msg.id))
            .set(iscompleted.eq(true))
            .get_result::<Todo>(&conn)
    }
}
impl Handler<GetAllTodos> for DbActor {
    type Result = QueryResult<Vec<Todo>>;

    fn handle(&mut self, _msg: GetAllTodos, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        todos.get_results::<Todo>(&conn)
    }
}
