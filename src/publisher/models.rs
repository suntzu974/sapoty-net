use serde::{Serialize,Deserialize};
use crate::schema::users;
use actix_web::{web};
use crate::Pool;
use diesel::prelude::*;

#[derive(Serialize,Deserialize,Queryable,Debug)]
pub struct User {
   pub id: i32,
   pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub name: String,
}
#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}




impl User {
    pub fn find_all(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let users: Vec<User> = users::table.load::<User>(&conn).expect("Error loading users");
        Ok(users)
    }
    pub fn find_by_id(pool: web::Data<Pool>,user_id:i32) -> Result<User, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let user: User = users::table.find(user_id).get_result::<User>(&conn).expect("Error loading user");
        Ok(user)
    }
    pub fn insert(
        db: web::Data<Pool>,
        user: web::Json<UserRequest>,
    ) -> Result<User, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_user = NewUser {
            name: &user.name,
        };
        let res = diesel::insert_into(users::table).values(&new_user).get_result(&conn)?;
        Ok(res)
    }
    
    pub fn delete(db: web::Data<Pool>, id: i32) -> Result<usize, diesel::result::Error> {
        let conn = db.get().unwrap();
        let count = diesel::delete(users::table.find(id)).execute(&conn)?;
        Ok(count)
    }
    
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

}
