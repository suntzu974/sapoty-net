#[macro_use]
extern crate diesel;
extern crate lazy_static;

use actix_web::{ middleware, web, App, HttpServer, Error, dev::ServiceRequest};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use core::pin::Pin;
use actix_storage::{Format, Storage};
use database::get_user_collection;
use crate::authentication::services::UserService;
use actix_cors::Cors;
use actix_web::http::header;
mod database;

mod stock;
mod review;
mod customer;
mod utils;
mod handlers;
mod schema;
mod student;
mod authentication;
mod publisher;
mod post;


pub struct ServiceContainer {
    user: UserService,
}

impl ServiceContainer {
    pub fn new(user: UserService) -> Self {
        ServiceContainer { user }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}
// create a middleware
 
async fn bearer_auth_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| Pin::new(data).get_ref().clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
fn validate_token(str: &str) -> Result<bool, std::io::Error>
{
    if str.eq("a-secure-token")
    {
        return Ok(true);
    }
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"));
}


  
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sled_database = std::env::var("SLED_DATABASE").expect("SLED DATABASE must be set");
    let ssl_key = std::env::var("SSL_KEY").expect("SSL KEY must be set");
    let ssl_crt = std::env::var("SSL_CRT").expect("SSL CRTY must be set");
    let documents_files = std::env::var("DOCUMENTS_FILES").expect("DOCUMENTS FILES  DIRECTORY NOT FOUND");
    let pictures_originals = std::env::var("PICTURES_ORIGINALS").expect("PICTURES ORIGNALS DIRECTORY NOT FOUND");
    let pictures_thumbnails = std::env::var("PICTURES_THUMBNAILS").expect("PICTURES THUMBNAILS DIRECTORY NOT FOUND");
    let pictures_web = std::env::var("PICTURES_WEB").expect("PICTURES WEB NOT FOUND");
    let qrcode_registration = std::env::var("QRCODE_REGISTRATION").expect("QRCODE REGISTRATION NOT FOUND");
    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("PORT").expect("PORT NOT FOUND");
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(ssl_key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(ssl_crt).unwrap();
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    

    std::fs::create_dir_all(documents_files).unwrap();
    std::fs::create_dir_all(pictures_originals).unwrap();
    std::fs::create_dir_all(pictures_thumbnails).unwrap();
    std::fs::create_dir_all(pictures_web).unwrap();
    std::fs::create_dir_all(qrcode_registration).unwrap();
    std::fs::create_dir_all(sled_database.clone()).unwrap();

    let store = actix_storage_sled::SledStore::from_db(
        actix_storage_sled::SledConfig::default()
            .path(sled_database.clone())
            .temporary(false)
            .open()?,
    );
   let storage = Storage::build().store(store).format(Format::Json).finish();
   let user_collection = get_user_collection();



    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin("http://0.0.0.0:3030")
        .allowed_origin("http://localhost:3030")
        .allowed_origin("http://localhost:8080")
        .allowed_methods(vec!["GET", "POST","DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600);
    
        let auth = HttpAuthentication::bearer(bearer_auth_validator);
        let service_container = ServiceContainer::new(UserService::new(user_collection.clone()));
        App::new()
        //.wrap(auth)
        .wrap(cors)
        .wrap(middleware::Logger::default())
        .data(web::JsonConfig::default().limit(4096))
        .data(pool.clone())
        .app_data(storage.clone())
        .data(AppState { service_container })
        .route("/", web::get().to(handlers::index))
        .configure(authentication::init_routes)
        .configure(stock::init_routes)
        .configure(review::init_routes)
        .configure(customer::init_routes)
        .configure(student::init_routes)
        .configure(publisher::init_routes)
        .configure(post::init_routes)
        .service(web::resource("/register").route(web::get().to(handlers::register)))
        .service(web::resource("/load").route(web::get().to(handlers::load)))
        .service(web::resource("/dist/{_:.*}").route(web::get().to(handlers::dist)))
        .service(web::resource("/registration").route(web::post().to(handlers::registration)))
        .service(web::resource("/ping").wrap(auth).route(web::get().to(handlers::ping)))
    })
    .bind_openssl(format!("{}:{}" ,host,port,), builder)?
    .run()
    .await
}
