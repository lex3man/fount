use actix_web::{main, web, App, HttpServer};
use configs::init;
use controllers::connect;
use routes::greeting;

mod configs;
mod controllers;
mod models;
mod routes;
#[cfg(test)]
mod tests;

#[main]
async fn main() -> std::io::Result<()> {
    let config = init();
    let port = config.get_port();
    connect(config);
    HttpServer::new(|| App::new().route("/hello", web::get().to(greeting)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
