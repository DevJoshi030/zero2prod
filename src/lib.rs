use actix_web::{
    dev::Server, get, post, web::Form, web::Json, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct Test<'a> {
    email: &'a str,
    name: &'a str,
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("I am up!!!!!")
}

#[post("/subscriptions")]
async fn subscriptions(_form: Json<FormData>) -> impl Responder {
    let t = Test {
        email: &_form.email,
        name: &_form.name,
    };
    HttpResponse::Ok().json(t)
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscriptions))
        .listen(listener)?
        .run();

    Ok(server)
}
