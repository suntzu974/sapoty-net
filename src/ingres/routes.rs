use crate::ingres::{ResponseApp,Tappent,Tapplig,Tnumbl};
use actix_web::{post,error,web, Error, HttpResponse};

#[post("/appro")]
pub async fn load(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {
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
    let obj = serde_json::from_slice::<Tapplig>(&body)?;
    let client = reqwest::blocking::Client::new();
    let stocks:ResponseApp = client.post(format!("{}://{}:{}/{}","http",ip_remote,50022,"stockingres"))
    .json(&obj)
    .send().unwrap()
    .json().unwrap();

    Ok(HttpResponse::Ok().json(stocks)) // <- send response
}

use futures::{StreamExt};
const MAX_SIZE: usize = 262_144; // max payload size is 256k
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(load);
}