use std::sync::RwLock;
use actix_web::{dev::Body, get, post, web, App, HttpResponse, HttpServer, Responder};
use web::*;

mod db;
use crate::db::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Tenken Web Server")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    Path((probe_id, _)): Path<(String, String)>,
    text: String,
    db: Data<RwLock<Tenken>>,
) -> impl Responder {
    db.write().unwrap()
        .put(probe_id, text);
    HttpResponse::Accepted().body(Body::Empty)
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    Path(probe_id): Path<String>,
    db: Data<RwLock<Tenken>>,
) -> impl Responder {
    db.read().unwrap()
        .get(probe_id)
        .map(|x| HttpResponse::Ok().body(x))
        .unwrap_or(HttpResponse::NotFound().body(Body::Empty))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Data::new(RwLock::new(Tenken::default()));
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await
}
