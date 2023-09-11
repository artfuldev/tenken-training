use actix_web::{get, Responder, HttpResponse};
use actix_web::web::{Path, Data};

use crate::actors::Tenken;

#[get("/probe/{probe_id}/latest")]
async fn latest_service(
    path: Path<String>,
    db: Data<Tenken>,
) -> impl Responder {
    let probe_id = path.into_inner();
    db.get(probe_id).await
        .map(|x| HttpResponse::Ok().content_type("application/json").body(x))
        .unwrap_or_else(|| HttpResponse::NotFound().body(()))
}
