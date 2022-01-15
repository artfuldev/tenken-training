use actix::{Actor, Addr};
use actix_web::{dev::Body, get, post, web, App, HttpResponse, HttpServer, Responder};
use web::*;

mod db;
use crate::db::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    Path((probe_id, _)): Path<(String, String)>,
    payload: String,
    db: Data<Addr<Tenken>>,
) -> impl Responder {
    db.do_send(ProbePayloadReceived::new(probe_id, payload));
    HttpResponse::Accepted().body(Body::Empty)
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    Path(probe_id): Path<String>,
    db: Data<Addr<Tenken>>,
) -> impl Responder {
    match db.send(ProbeRequestReceived::new(probe_id)).await {
        Ok(response) =>
            match response {
                Ok(result) =>
                    result
                        .map(|x| HttpResponse::Ok().content_type("application/json").body(x))
                        .unwrap_or(HttpResponse::NotFound().body(Body::Empty)),
                Err(_) => HttpResponse::InternalServerError().body(Body::Empty)
            },
        Err(_) => HttpResponse::InternalServerError().body(Body::Empty)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Data::new(Tenken::default().start());
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
