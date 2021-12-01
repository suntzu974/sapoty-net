use serde::{Deserialize, Serialize};
use crate::diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::schema::reviews;
use actix_web::{web};
use crate::Pool;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Review {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub original: String,
    #[serde(rename="url")]
    pub thumbnail: String,
    pub web: String,
    pub deleted: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "reviews"]
pub struct NewReview<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub original: &'a str,
    pub thumbnail: &'a str,
    pub web: &'a str,
    pub deleted: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputReview {
    pub title: String,
    pub description: String,
    pub original: String,
    pub thumbnail: String,
    pub web: String,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Passphrase {
    pub passphrase: String,
}

impl Review {
    pub fn get_by_id(pool: web::Data<Pool>, review_id: i32) -> Result<Review, diesel::result::Error> {
        let conn = pool.get().unwrap();
        reviews::table.find(review_id).get_result::<Review>(&conn)
    }
    
    pub fn get(pool: web::Data<Pool>) -> Result<Vec<Review>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let items = reviews::table.load::<Review>(&conn)?;
        Ok(items)
    }
    
    pub fn insert(
        db: web::Data<Pool>,
        item: web::Json<InputReview>,
    ) -> Result<Review, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_review = NewReview {
            title: &item.title,
            description: &item.description,
            original: &item.original,
            thumbnail: &item.thumbnail,
            web: &item.web,
            deleted: item.deleted,
            created_at: chrono::Local::now().naive_local(),
        };
        let res = diesel::insert_into(reviews::table).values(&new_review).get_result(&conn)?;
        Ok(res)
    }
    
    pub fn delete(db: web::Data<Pool>, review_id: i32) -> Result<usize, diesel::result::Error> {
        let conn = db.get().unwrap();
        let count = diesel::delete(reviews::table.find(review_id)).execute(&conn)?;
        Ok(count)
    }
    
    pub fn update(
        db: web::Data<Pool>,
        review_id: i32,
        item: web::Json<InputReview>) -> Result<Review, diesel::result::Error> {
        let conn = db.get().unwrap();
        let updated_review = NewReview {
            title: &item.title,
            original: &item.original,
            thumbnail: &item.thumbnail,
            web: &item.web,
            description: &item.description,
            deleted: item.deleted,
            created_at: chrono::Local::now().naive_local(),
        };
        let res = diesel::update(reviews::table)
        .filter(reviews::id.eq(review_id))
        .set(&updated_review)
        .get_result(&conn)?;
    
        Ok(res)
    }


}