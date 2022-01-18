use actix::Addr;
use actix_web::{post, Responder};
use actix_web::web::{Path, HttpResponse, Data};

use crate::messages::WriteRequested;
use crate::actors::Tenken;

#[post("/probe/{probe_id}/event/{event_id}")]
async fn write_service(
    path: Path<(String, String)>,
    payload: String,
    db: Data<Addr<Tenken>>,
) -> impl Responder {
    let (probe_id, _) = path.into_inner();
    db.do_send(WriteRequested { key: probe_id, value: payload });
    HttpResponse::Accepted().body(())
}
