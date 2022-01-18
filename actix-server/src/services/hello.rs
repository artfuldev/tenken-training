use actix_web::{get, Responder};
use actix_web::web::{HttpResponse};

#[get("/")]
async fn hello_service() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
