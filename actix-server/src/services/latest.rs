use std::sync::Mutex;
use actix_web::{get, Responder};
use actix_web::web::{Path, HttpResponse, Data};

use crate::actors::Tenken;

#[get("/probe/{probe_id}/latest")]
async fn latest_service(
    path: Path<String>,
    db: Data<Mutex<Tenken>>,
) -> impl Responder {
    let probe_id = path.into_inner();
    db.lock().unwrap().get(probe_id).await
        .map(|x| HttpResponse::Ok().content_type("application/json").body(x))
        .unwrap_or_else(|| HttpResponse::NotFound().body(()))
}
