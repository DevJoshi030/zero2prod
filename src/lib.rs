use actix_web::{dev::Server, get, App, HttpServer, Responder};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    "I am up!!!!!"
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .listen(listener)?
        .run();

    Ok(server)
}
