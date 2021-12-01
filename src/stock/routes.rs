use crate::stock::{ResponseStock,QueryStock,QueryTransfert,ResponseAppro};
use actix_web::{post,error,web, Error, HttpResponse};

use futures::{StreamExt};
const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/stockingres")]
pub async fn query(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {
    let ip_remote = std::env::var("IP_REMOTE").expect("PICTURES DIRECTORY NOT FOUND");
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryStock>(&body)?;
    let client = reqwest::blocking::Client::new();
    let stocks:ResponseStock = client.post(format!("{}://{}:{}/{}","http",ip_remote,50022,"stockingres"))
    .json(&obj)
    .send().unwrap()
    .json().unwrap();

    Ok(HttpResponse::Ok().json(stocks)) // <- send response
}
#[post("/stocks")]
pub async fn query_by_gencod(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {
    let ip_remote = std::env::var("IP_REMOTE").expect("PICTURES DIRECTORY NOT FOUND");
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryStock>(&body)?;
    let client = reqwest::blocking::Client::new();
    let stocks:ResponseStock = client.post(format!("{}://{}:{}/{}","http",ip_remote,50022,"stockingres"))
    .json(&obj)
    .send().unwrap()
    .json().unwrap();

    Ok(HttpResponse::Ok().json(stocks)) // <- send response
}

#[post("/transfert")]
pub async fn build_appro(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {
    let ip_remote = std::env::var("IP_REMOTE").expect("IP REMOTE NOT FOUND");
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryTransfert>(&body)?;
    let client = reqwest::blocking::Client::new();
    let appro:ResponseAppro = client.post(format!("{}://{}:{}/{}","http",ip_remote,50022,"transfert"))
    .json(&obj)
    .send().unwrap()
    .json().unwrap();

    Ok(HttpResponse::Ok().json(appro)) // <- send response
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(query);
    cfg.service(query_by_gencod);
    cfg.service(build_appro);
}