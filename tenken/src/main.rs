use actix::{Actor, Addr};
use actix_web::{get, post, App, HttpServer, Responder};
use actix_web::web::{Path, HttpResponse, Data};

mod db;
mod writer;
mod tick;
use crate::db::*;
use crate::writer::Writer;
use crate::tick::Tick;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    path: Path<(String, String)>,
    payload: String,
    db: Data<Addr<Tenken>>,
) -> impl Responder {
    let (probe_id, _) = path.into_inner();
    db.do_send(ProbePayloadReceived::new(probe_id, payload));
    HttpResponse::Accepted().body(())
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    path: Path<String>,
    db: Data<Addr<Tenken>>,
) -> impl Responder {
    let probe_id = path.into_inner();
    match db.send(ProbeRequestReceived::new(probe_id)).await {
        Ok(response) =>
            match response {
                Ok(result) =>
                    result
                        .map(|x| HttpResponse::Ok().content_type("application/json").body(x))
                        .unwrap_or(HttpResponse::NotFound().body(())),
                Err(_) => HttpResponse::InternalServerError().body(())
            },
        Err(_) => HttpResponse::InternalServerError().body(())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let writer = Writer::default().start();
    let _ = Tick::new(writer.clone()).start();
    let db = Data::new(Tenken::new(writer.clone()).start());
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
