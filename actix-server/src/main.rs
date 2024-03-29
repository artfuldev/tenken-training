use actix_web::{App, HttpServer};
use actix_web::web::Data;

mod messages;
mod actors;
mod services;
use crate::actors::*;
use crate::services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tenken = Tenken::new(1_200_000);
    let db = Data::new(tenken);
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(hello_service)
            .service(latest_service)
            .service(write_service)
    })
    .bind("0.0.0.0:8080")?
    .workers(2)
    .run()
    .await
}
