use actix::{Addr, Actor};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chashmap::CHashMap;
use std::io::{self, *};
use std::fs::{OpenOptions, File};
use std::ops::Deref;
use web::*;
use serde_json::{to_string, from_str};

mod dto;
mod writer;
use crate::dto::{ProbeData, ProbeRequest, ProbeResponse};
use crate::writer::{Writer, ProbePayloadReceived};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Tenken Web Server")
}

#[post("/probe/{probe_id}/event/{event_id}")]
async fn store_message(
    Path((probe_id, _)): Path<(String, String)>,
    json: Json<ProbeRequest>,
    cache: Data<CHashMap<String, ProbeData>>,
    writer: Data<Addr<Writer>>
) -> impl Responder {
    let data = ProbeData::from(json.into_inner());
    let result = writer.send(ProbePayloadReceived::new(probe_id.clone(), data.clone())).await.unwrap();
    match result {
        Ok(_) =>  {
            cache.insert(probe_id, data);
            HttpResponse::Accepted().body("")
        },
        Err(e) => {
            let body = format!("Couldn't write to file: {:?}", e);
            eprintln!("{}", body);
            HttpResponse::InternalServerError().body(body)
        }
    }
}

#[get("/probe/{probe_id}/latest")]
async fn get_message(
    Path(probe_id): Path<String>,
    cache: Data<CHashMap<String, ProbeData>>,
) -> impl Responder {
      cache
        .get(&probe_id)
        .map(|x| x.deref().clone())
        .map(ProbeResponse::from)
        .map(|x| to_string(&x))
        .map(|x| match x { Ok(y) => Some(y), Err(_) => None })
        .flatten()
        .map(|x| HttpResponse::Ok().body(x))
        .unwrap_or(HttpResponse::NotFound().body(""))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<std::path::Path>, {
    let file = OpenOptions::new().read(true).create(true).open(filename.as_ref())?;
    Ok(io::BufReader::new(file).lines())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _cache = CHashMap::new();
    if let Ok(lines) = read_lines("db.dat") {
        for line in lines {
            if let Ok(entry) = line {
                let splits = entry.split(":::").collect::<Vec<&str>>();
                let key = splits[0];
                let value = splits[1];
                println!("reading probe id {} data {}", key, value);
                let data: ProbeData = from_str(value).unwrap();
                _cache.insert(key.to_string(), data);
            }
        }
    }
    let writer = Data::new(Writer::new()?.start());
    let cache = Data::new(_cache);
    HttpServer::new(move || {
        let eight_bytes = 8192;
        let json_config = JsonConfig::default().limit(eight_bytes);

        App::new()
            .app_data(json_config)
            .app_data(cache.clone())
            .app_data(writer.clone())
            .service(hello)
            .service(store_message)
            .service(get_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
