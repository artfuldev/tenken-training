use std::sync::Mutex;
use actix_web::{dev::Body, get, post, web, App, HttpResponse, HttpServer, Responder};
use fxhash::FxHashMap;
use web::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Tenken Web Server")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    Path((probe_id, _)): Path<(String, String)>,
    text: String,
    cache: Data<Mutex<FxHashMap<String, String>>>,
) -> impl Responder {
    cache.lock().unwrap().insert(probe_id, text);
    HttpResponse::Accepted().body(Body::Empty)
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    Path(probe_id): Path<String>,
    cache: Data<Mutex<FxHashMap<String, String>>>,
) -> impl Responder {
    cache
        .lock()
        .unwrap()
        .get(&probe_id)
        .map(|x| x.clone())
        .map(|x| HttpResponse::Ok().body(x))
        .unwrap_or(HttpResponse::NotFound().body(Body::Empty))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cache = Data::new(Mutex::new(FxHashMap::<String, String>::default()));
    HttpServer::new(move || {
        App::new()
            .app_data(cache.clone())
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}
