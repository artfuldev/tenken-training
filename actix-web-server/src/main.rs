use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;

mod message;
use crate::message::Message;

struct AppState {
    in_memory_store: chashmap::CHashMap<String, Message>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Tenken Web Server")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    web::Path((probe_id, event_id)): web::Path<(String, String)>,
    message: web::Json<Message>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("my-file")
        .unwrap();
    if let Err(e) = writeln!(file, "{}, {:?}", probe_id, message) {
        eprintln!("Couldn't write to file: {}", e);
    }
    HttpResponse::Ok().body("Created")
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    web::Path(probe_id): web::Path<String>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let data = app_data.get_ref().in_memory_store.get(&probe_id);
    HttpResponse::Ok().body(data.map(|x| x.deref().probeId.clone()).unwrap_or_default())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let eight_bytes = 8192;
        let json_config = web::JsonConfig::default().limit(eight_bytes);
        let init_data = web::Data::new(AppState {
            in_memory_store: chashmap::CHashMap::new(),
        });

        App::new()
            .app_data(json_config)
            .app_data(init_data)
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
