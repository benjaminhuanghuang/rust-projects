use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[path = "../errors.rs"]
mod errors;

#[path = "../dbaccess/mod.rs"]
mod db_access;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../models/mod.rs"]
mod models;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  // read .env config file
  let database_url = env::var("DATABASE_URL").expect("can not find DATABASE_URL in .env");
  // connect to db
  let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();


  let shared_data = web::Data::new(AppState {
    health_check_response: "I'm OK.".to_string(),
    visit_count: Mutex::new(0),
    db: db_pool
  });

  let app = move || {
    App::new()
      .app_data(shared_data.clone())
      .configure(general_routes)
      .configure(course_routes)
  };
  HttpServer::new(app).bind("127.0.0.1:8964")?.run().await
}
