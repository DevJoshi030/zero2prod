use actix_web::{dev::Server, get, post, web::Form, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I am up!!!!!")
}

#[post("/subscriptions")]
async fn subscriptions(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscriptions))
        .listen(listener)?
        .run();

    Ok(server)
}
