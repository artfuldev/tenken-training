use std::sync::Mutex;
use actix_web::{post, Responder};
use actix_web::web::{Path, HttpResponse, Data};
use crate::actors::Tenken;

#[post("/probe/{probe_id}/event/{event_id}")]
async fn write_service(
    path: Path<(String, String)>,
    payload: String,
    db: Data<Mutex<Tenken>>,
) -> impl Responder {
    let (probe_id, _) = path.into_inner();
    db.lock().unwrap().put(probe_id, payload);
    HttpResponse::Accepted().body(())
}
