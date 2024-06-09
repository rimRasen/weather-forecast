use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish() // finish() to get an empty body 200 Ok.
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String, 
    name: String
}

async fn subscribe(_form : web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
let server = HttpServer::new(|| {
App::new()
.route("/health_check", web::get().to(health_check))
.route("/subscriptions", web::post().to(subscribe))
})
.listen(listener)?
.run();

Ok(server)
}