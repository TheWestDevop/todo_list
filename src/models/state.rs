use crate::actors::todo_actors::DbActor;
use crate::actix::Addr;


pub struct AppState {
    pub db: Addr<DbActor>
 }
 impl AppState {
     pub fn new(db: Addr<DbActor>) -> AppState {
         AppState {
              db
          }
      }
  }