use actix_web::{HttpResponse, Responder};

pub async fn greeting() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
