use actix_web::{main, web, App, HttpServer};
use configs::get_port;
use routes::greeting;

mod configs;
mod controllers;
mod models;
mod routes;
#[cfg(test)]
mod tests;

#[main]
async fn main() -> std::io::Result<()> {
    let port = get_port();
    HttpServer::new(|| App::new().route("/hello", web::get().to(greeting)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
