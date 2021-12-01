use crate::review::{Review,InputReview,File};
use actix_web::{delete,get,post,put,web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use actix_multipart::Multipart;
use crate::Pool;
use crate::utils::utils::{resize_for_thumbnail,resize_for_web};
use std::fs;
use std::io::Write;
use std::path::Path;

#[get("/reviews")]
async fn find_all(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Review::get(db))
        .await
        .map(|review| HttpResponse::Ok().json(review))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /users/{id}
#[get("/reviews/{id}")]
async fn find_by_id(
    db: web::Data<Pool>,
    review_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Review::get_by_id(db, review_id.into_inner()))
            .await
            .map(|review| HttpResponse::Ok().json(review))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[post("/reviews")]
pub async fn create(
    db: web::Data<Pool>,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Review::insert(db, item))
        .await
        .map(|review| HttpResponse::Created().json(review))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /reviews/{id}
#[delete("/reviews/{id}")]
pub async fn remove(
    db: web::Data<Pool>,
    review_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Review::delete(db, review_id.into_inner()))
            .await
            .map(|review| HttpResponse::Ok().json(review))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[put("/reviews/{id}")]
pub async fn put(
    db: web::Data<Pool>,
    review_id: web::Path<i32>,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Review::update(db,review_id.into_inner(),item))
            .await
            .map(|review| HttpResponse::Ok().json(review))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
#[post("/upload")]
pub async fn upload(db: web::Data<Pool>,mut payload: Multipart) -> Result<HttpResponse, Error> {
    let pictures_originals = std::env::var("PICTURES_ORIGINALS").expect("PICTURES DIRECTORY NOT FOUND");
    let host = std::env::var("HOST").expect("HOST NOT FOUND");
    let port = std::env::var("PORT").expect("HOST NOT FOUND");
    fs::create_dir_all(pictures_originals.to_string())?;
    let mut filename = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        filename = format!("{}", content_type.get_filename().unwrap(), );
        let filepath = format!("{}/{}", pictures_originals, sanitize_filename::sanitize(&filename));
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }
    }
    let urlpath = format!("{}://{}:{}/download/{}","https",host,port,sanitize_filename::sanitize(&filename));
    let new_review = InputReview {
        title: filename.clone(),
        description: filename.clone(),
        original: urlpath,
        thumbnail : resize_for_thumbnail(filename.clone()).await,
        web : resize_for_web(filename.clone()).await,
        deleted: false,
    };

    Ok(web::block(move || Review::insert(db, web::Json(new_review)))
        .await
        .map(|review| HttpResponse::Created().json(review))
        .map_err(|_| HttpResponse::InternalServerError())?)

}
#[get("/download/{name}")]
pub async fn download(info: web::Path<File>) -> HttpResponse {
    let pictures_originals = std::env::var("PICTURES_ORIGINALS").expect("PICTURES ORIGINALS NOT FOUND");
    let path = format!("{}/{}", pictures_originals.to_string(), info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body(data)
}
#[get("/thumbnail/{name}")]
pub async fn thumbnail(info: web::Path<File>) -> HttpResponse {
    let pictures_thumbnails = std::env::var("PICTURES_THUMBNAILS").expect("PICTURES DIRECTORY NOT FOUND");
    let path = format!("{}/{}", pictures_thumbnails.to_string(), info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body(data)
}
#[get("/web/{name}")]
pub async fn webpicture(info: web::Path<File>) -> HttpResponse {
    let pictures_web = std::env::var("PICTURES_WEB").expect("PICTURES WEB NOT FOUND");
    let path = format!("{}/{}", pictures_web.to_string(), info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body(data)
}

#[delete("/removeThumbnail/{name}")]
pub async fn remove_download(info: web::Path<File>) -> HttpResponse {
    let pictures_originals = std::env::var("PICTURES_ORIGINALS").expect("PICTURES ORIGINALS NOT FOUND");
    let path = format!("{}/{}", pictures_originals.to_string(), info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
        });
    }
    fs::remove_file(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body("File deleted")
}

#[delete("/removeThumbnail/{name}")]
pub async fn remove_thumbnail(info: web::Path<File>) -> HttpResponse {
    let pictures_thumbnails = std::env::var("PICTURES_THUMBNAILS").expect("PICTURES DIRECTORY NOT FOUND");
    let path = format!("{}/{}", pictures_thumbnails.to_string(), info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
        });
    }
    fs::remove_file(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body("file deleted")
}
#[delete("/removeWeb/{name}")]
pub async fn remove_web(info: web::Path<File>) -> HttpResponse {
    let pictures_web = std::env::var("PICTURES_WEB").expect("PICTURES DIRECTORY NOT FOUND");
    let path = format!("{}/{}", pictures_web.to_string(), info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
        });
    }
    fs::remove_file(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body("file deleted")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(create);
    cfg.service(put);
    cfg.service(remove);
    cfg.service(upload);
    cfg.service(download);
    cfg.service(thumbnail);
    cfg.service(webpicture);
    cfg.service(remove_download);
    cfg.service(remove_thumbnail);
    cfg.service(remove_web);
}