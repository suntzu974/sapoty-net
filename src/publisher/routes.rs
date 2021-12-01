use crate::publisher::{User,UserRequest};
use actix_web::{get,put,post,delete,web, Error, HttpResponse};
use crate::Pool;

#[get("/publishers")]
async fn find_all (
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || User::find_all(db))
            .await
            .map(|posts| HttpResponse::Ok().json(posts))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
#[get("/publishers/{id}")]
async fn find_by_id (
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || User::find_by_id(db,user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )    
}

#[post("/publishers")]
pub async fn create (
    db: web::Data<Pool>,
    user: web::Json<UserRequest>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || User::insert(db, user))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[delete("/publishers/{id}")]
pub async fn remove (
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || User::delete(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
#[put("/publishers/{id}")]
pub async fn update (
    db: web::Data<Pool>,
    user_for_updating: web::Json<UserRequest>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || User::update(db,user_for_updating,user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(create);
    cfg.service(update);
    cfg.service(remove);
}
