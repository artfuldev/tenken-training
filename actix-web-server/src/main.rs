use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chashmap::CHashMap;
use std::io::{self, *};
use std::fs::{OpenOptions, File};
use std::ops::Deref;
use web::*;
use serde_json::{to_string, from_str};

mod message;
use crate::message::Message;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Tenken Web Server")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    Path((probe_id, _)): Path<(String, String)>,
    json: Json<Message>,
    app_data: Data<CHashMap<String, Message>>,
) -> impl Responder {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("db.dat")
        .unwrap();
    let message = json.into_inner();
    match writeln!(file, "{}:::{}", probe_id, to_string(&message).unwrap()) {
        Ok(_) =>  {
            app_data.insert(probe_id, message);
            HttpResponse::Ok().body("Created")
        },
        Err(e) => {
            let body = format!("Couldn't write to file: {}", e);
            eprintln!("{}", body);
            HttpResponse::InternalServerError().body(body)
        }
    }
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    Path(probe_id): Path<String>,
    app_data: Data<CHashMap<String, Message>>,
) -> impl Responder {
    let data = app_data.get(&probe_id);
    HttpResponse::Ok().body(data.map(|x| x.deref().eventId.clone()).unwrap_or_default())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<std::path::Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = CHashMap::new();
    if let Ok(lines) = read_lines("db.dat") {
        for line in lines {
            if let Ok(entry) = line {
                let splits = entry.split(":::").collect::<Vec<&str>>();
                let key = splits[0];
                let value = splits[1];
                println!("reading probe id {} message {}", key, value);
                let message: Message = from_str(value).unwrap();
                state.insert(key.to_string(), message);
            }
        }
    }
    let initial_state = Data::new(state);
    HttpServer::new(move || {
        let eight_bytes = 8192;
        let json_config = JsonConfig::default().limit(eight_bytes);

        App::new()
            .app_data(json_config)
            .app_data(initial_state.clone())
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
