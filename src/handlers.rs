extern crate image;

use super::Pool;
use actix_web::{web, Error, HttpResponse, body::Body};
use askama::Template;
use qrcode::QrCode;
use image::Luma;
use sha256::digest;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use crate::review::{Review,Passphrase};

#[derive(Template)]
#[template(path = "index.html")] 
struct ReviewlistTemplate<'a> {
    reviews: &'a Vec<Review>,
}

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate<'a> {
    identification: &'a str,
    sha256:&'a str,
}

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
      Some(content) => {
        let body: Body = match content.data {
          Cow::Borrowed(bytes) => bytes.into(),
          Cow::Owned(bytes) => bytes.into(),
        };
        HttpResponse::Ok().content_type(from_path(path).first_or_octet_stream().as_ref()).body(body)
      }
      None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
pub async fn  register() -> HttpResponse {
    handle_embedded_file("register.html")
}

pub async fn  load() -> HttpResponse {
    handle_embedded_file("load.html")
}


pub async fn dist(path: web::Path<String>) -> HttpResponse {
   handle_embedded_file(&path.0)
}

pub async fn registration(params: web::Form<Passphrase>) -> Result<HttpResponse, Error>  {
    let val = digest(&params.passphrase.to_string());
    let code = QrCode::new(&val).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let qrcode_registration = std::env::var("QRCODE_REGISTRATION").expect("QRCODE REGISTRATON NOT FOUND");

    let path = format!("{}/{}.png", qrcode_registration.to_string(), &params.passphrase.to_string());
    image.save(&path).unwrap();

    let template = RegistrationTemplate {
        identification: &params.passphrase,
        sha256:&val,
    };
    let res = template
        .render()
        .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(res))

}

pub async fn index(db: web::Data<Pool>) -> Result<HttpResponse, Error>  {
    let mut pictures = Vec::new();
    match Review::get(db) {
        Ok(data) => {
            pictures = data;//.iter().map(|p| p.original.clone()).collect();
        },
        Err(e) => println!("error parsing header: {:?}", e),
    }
    let template = ReviewlistTemplate {
        reviews: &pictures,
    };
    let res = template
        .render()
        .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(res))
}

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .body("pong!".to_string())
}

