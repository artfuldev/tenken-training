use actix_web::{get, Responder, HttpResponse};

#[get("/")]
async fn hello_service() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
