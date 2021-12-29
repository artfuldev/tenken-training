use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chashmap::CHashMap;
use std::io::*;
use std::fs::OpenOptions;
use std::ops::Deref;

mod message;
use crate::message::Message;

#[derive(Clone)]
struct AppState {
    in_memory_store: CHashMap<String, Message>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Tenken Web Server")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    web::Path((probe_id, _)): web::Path<(String, String)>,
    message: web::Json<Message>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("db.dat")
        .unwrap();
    match writeln!(file, "{}, {:?}", probe_id, message) {
        Ok(_) =>  {
            app_data.in_memory_store.insert(probe_id, message.into_inner());
            HttpResponse::Ok().body("Created")
        },
        Err(e) => {
            let body = format!("Couldn't write to file: {}", e);
            eprintln!("{}", body);
            HttpResponse::InternalServerError().body(body)
        }
    }
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    web::Path(probe_id): web::Path<String>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let data = app_data.in_memory_store.get(&probe_id);
    HttpResponse::Ok().body(data.map(|x| x.deref().eventId.clone()).unwrap_or_default())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_state = web::Data::new(AppState { in_memory_store: CHashMap::new() });
    HttpServer::new(move || {
        let eight_bytes = 8192;
        let json_config = web::JsonConfig::default().limit(eight_bytes);

        App::new()
            .app_data(json_config)
            .app_data(initial_state.clone())
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
