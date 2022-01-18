use actix::Addr;
use actix_web::{get, Responder};
use actix_web::web::{Path, HttpResponse, Data};

use crate::actors::Tenken;
use crate::messages::LatestRequested;

#[get("/probe/{probe_id}/latest")]
async fn latest_service(
    path: Path<String>,
    db: Data<Addr<Tenken>>,
) -> impl Responder {
    let probe_id = path.into_inner();
    match db.send(LatestRequested(probe_id)).await {
        Ok(result) =>
            result
                .map(|x| HttpResponse::Ok().content_type("application/json").body(x))
                .unwrap_or(HttpResponse::NotFound().body(())),
        Err(_) => HttpResponse::InternalServerError().body(())
    }
}
