use actix_web::{post, Responder, HttpResponse};
use actix_web::web::{Path, Data};
use crate::actors::Tenken;

#[post("/probe/{probe_id}/event/{event_id}")]
async fn write_service(
    path: Path<(String, String)>,
    payload: String,
    db: Data<Tenken>,
) -> impl Responder {
    let (probe_id, _) = path.into_inner();
    db.put(probe_id, payload);
    HttpResponse::Accepted().body(())
}
