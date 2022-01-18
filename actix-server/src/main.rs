use actix::Actor;
use actix_web::{App, HttpServer};
use actix_web::web::Data;

mod messages;
mod actors;
mod services;
use crate::actors::*;
use crate::services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db =
        Data::new(
            Tenken::new(
                "db.dat".to_string(),
                1_500_000, // 1.5 million probes
                2 * 1024, // 2 KB
                false
            ).start()
        );
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
