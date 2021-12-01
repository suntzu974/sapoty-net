use crate::post::{Post,PostRequest};
use actix_web::{get,web,delete,post,put, Error, HttpResponse};
use crate::Pool;

#[get("/posts")]
async fn find_all (
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Post::find_by_all(db))
            .await
            .map(|posts| HttpResponse::Ok().json(posts))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
#[get("/publisher/{id}/posts")]
async fn find_by_id (
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Post::find_by_id(db,user_id.into_inner()))
            .await
            .map(|posts| HttpResponse::Ok().json(posts))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[get("/publisher/{user_id}/post/{post_id}")]
async fn find_post_by_user_id (
    db: web::Data<Pool>,
    web::Path((user_id, post_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Post::find_post_by_user_id(db,user_id,post_id))
            .await
            .map(|posts| HttpResponse::Ok().json(posts))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
#[post("/publisher/{user_id}/post")]
pub async fn create (
    db: web::Data<Pool>,
    post: web::Json<PostRequest>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Post::insert(db, post,user_id.into_inner() ))
        .await
        .map(|post| HttpResponse::Created().json(post))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[put("/publisher/{user_id}/post/{post_id}")]
async fn update (
    db: web::Data<Pool>,
    updated_post: web::Json<PostRequest>,
    web::Path((user_id, post_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Post::update(db,updated_post,user_id,post_id))
            .await
            .map(|posts| HttpResponse::Ok().json(posts))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[delete("/publisher/{user_id}/post/{post_id}")]
async fn remove (
    db: web::Data<Pool>,
    web::Path((user_id, post_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || Post::delete(db,user_id,post_id))
            .await
            .map(|posts| HttpResponse::Ok().json(posts))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(find_post_by_user_id);
    cfg.service(remove);
    cfg.service(create);
    cfg.service(update);

}
