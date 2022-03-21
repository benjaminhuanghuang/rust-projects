use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// config route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/health", web::get().to(health_check_handler));
}

// handler
pub async fn health_check_handler() -> impl Responder {
  HttpResponse::Ok().json("Actix web service is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let app = move || App::new().configure(general_routes);

  HttpServer::new(app).bind("127.0.0.1:8964")?.run().await
}
