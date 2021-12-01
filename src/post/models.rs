use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::schema::posts;
use serde::{Serialize,Deserialize};
use actix_web::{web};
use crate::Pool;
use crate::diesel::RunQueryDsl;
use diesel::dsl::sql;
use diesel::sql_types::Bool;

#[derive(Serialize,Deserialize,Queryable,Debug)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
    pub title: String,
}
#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub title: &'a str,
}
#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "posts"]
pub struct UpdatedPost<'a> {
    pub title: &'a str,
}

impl Post {
   
    pub fn find_by_all(pool: web::Data<Pool>) -> Result<Vec<Post>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let my_posts: Vec<Post> = posts::table.load(&conn).expect("Error loading posts");
        Ok(my_posts)
    }
    pub fn find_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<Vec<Post>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let query = posts::table.filter(sql::<Bool>(format!("user_id={} ",user_id).as_str()));
//        let query = sql::<(Integer, Integer, Text)>(format!("SELECT id, user_id, title FROM posts WHERE user_id={};", user_id).as_str());
        let posts = query.load::<Post>(&conn).expect("Can't query users");
        Ok(posts)
    }
    pub fn find_post_by_user_id(pool: web::Data<Pool>, user_id: i32,post_id:i32) -> Result<Vec<Post>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let query = posts::table.filter(sql::<Bool>(format!("id={} and user_id={} ",post_id,user_id).as_str()));
        let posts = query.load::<Post>(&conn).expect("Can't query posts");
        Ok(posts)
    }
    pub fn insert(
        db: web::Data<Pool>,
        post: web::Json<PostRequest>,
        user_id: i32,
    ) -> Result<Post, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_post = NewPost {
            user_id:user_id,
            title: &post.title,
        };
        let res = diesel::insert_into(posts::table).values(&new_post).get_result(&conn)?;
        Ok(res)
    }
    /*
    pub fn update(
        db: web::Data<Pool>,
        user_for_update: web::Json<UserRequest>,
        user_id: i32
        ) -> Result<User, diesel::result::Error> {
        let conn = db.get().unwrap();
        let updated_user = NewUser {
            name: &user_for_update.name,
        };
        let res = diesel::update(users::table)
        .filter(users::id.eq(user_id))
        .set(&updated_user)
        .get_result(&conn)?;
    
        Ok(res)
    }
    */ 
    pub fn update(db: web::Data<Pool>, post_for_update: web::Json<PostRequest>,user_id: i32, post_id:i32) -> Result<Post, diesel::result::Error> {
        let conn = db.get().unwrap();
        let updated_post = UpdatedPost {
            title: &post_for_update.title,
        };
        let res = diesel::update(posts::table)
//        .filter(posts::id.eq(post_id))
        .filter(sql::<Bool>(format!("id={} and user_id={}",post_id,user_id).as_str()))
        .set(&updated_post)
        .get_result(&conn)?;

        Ok(res)
    }

    pub fn delete(db: web::Data<Pool>, user_id: i32, post_id:i32) -> Result<usize, diesel::result::Error> {
        let conn = db.get().unwrap();
        let query = posts::table.filter(sql::<Bool>(format!("id={} and user_id={} ",post_id,user_id).as_str()));
        let count = diesel::delete(query).execute(&conn)?;
        Ok(count)
    }
    
}
