extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod actors;
mod middleware;
mod models;
mod routes;
mod schema;
mod utils;

use actix_web::{App, HttpServer,  middleware::{Logger}, web};

use models::state::AppState;
use routes::route::*;
use routes::utils_route::*;
use tracing::instrument;
use utils::{config::boot};

#[actix_web::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    let boot = boot().await;
    let boot_copy = boot.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{authorization}i"))
            
            .service(health)
            .service(
                web::scope("/todo")
                    .service(get_todos)
                    .service(create_todo)
                    .service(update_todo)
                    .service(todo_completed)
                    .service(delete_todo),
            )
            .default_service(
                web::route().to(route_not_found), //
            )
            .data(AppState::new(boot.db.clone()))
    })
    .workers(boot_copy.workers)
    .bind(format!("{}:{}", boot_copy.host, boot_copy.port))?
    .run()
    .await
}
