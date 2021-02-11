extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;



mod utils;
mod models;
mod schema;
mod actors;
mod routes;

use actix::SyncArbiter;
use actix_web::{ HttpServer, App, web};
use utils::db::{get_pool, run_migrations};
use std::env;
use dotenv::dotenv;
use actors::todo_actors::DbActor;
use routes::route::*;
use models::state::AppState;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;


#[actix_web::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");
    
    let host = env::var("HOST").expect("Error Host");
    let port = env::var("PORT").expect("Error PORT");
    
    info!("Starting server at http://{}:{}/", host,port);
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(10, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(health)
            .service(
                web::scope("/todo")
            .service(get_todos)
            .service(create_todo)
            .service(update_todo)
            .service(todo_completed)
            .service(delete_todo)
            )
            .data(AppState::new(db_addr.clone()))
    }).workers(10)
    .bind(format!("{}:{}",host,port))?
    .run()
    .await
}
