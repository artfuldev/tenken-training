use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chashmap::CHashMap;
use std::ops::Deref;
use web::*;
use serde_json::{to_string};
mod dto;
use crate::dto::{ProbeData, ProbeRequest, ProbeResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Tenken Web Server")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    Path((probe_id, _)): Path<(String, String)>,
    json: Json<ProbeRequest>,
    cache: Data<CHashMap<String, ProbeData>>,
) -> impl Responder {
    let data = ProbeData::from(json.into_inner());
    cache.insert(probe_id, data);
    HttpResponse::Accepted().body("")
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    Path(probe_id): Path<String>,
    cache: Data<CHashMap<String, ProbeData>>,
) -> impl Responder {
      cache
        .get(&probe_id)
        .map(|x| x.deref().clone())
        .map(ProbeResponse::from)
        .map(|x| to_string(&x))
        .map(|x| match x { Ok(y) => Some(y), Err(_) => None })
        .flatten()
        .map(|x| HttpResponse::Ok().body(x))
        .unwrap_or(HttpResponse::NotFound().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cache = Data::new(CHashMap::<String, ProbeData>::new());
    HttpServer::new(move || {
        App::new()
            .app_data(cache.clone())
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("127.0.0.1:8080")?
    .workers(8)
    .run()
    .await
}
