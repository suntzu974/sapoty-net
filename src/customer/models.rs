use serde::{Deserialize, Serialize};
use crate::diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::schema::customers;
use actix_web::{web};
use crate::Pool;

#[derive(Debug, Serialize, Deserialize, Queryable,Insertable, AsChangeset)]
pub struct Customer {
//    #[serde(rename="i4numcli")]
    pub refcli:i32,
 //   #[serde(rename="vcrsoc1")]
    pub name: String,
 //   #[serde(rename="vcadr1")]
    pub address: String,
 //   #[serde(rename="chpos")]
    pub postal: String,
 //   #[serde(rename="vcville")]
    pub town: String,

}
#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "customers"]
pub struct NewCustomer<'a> {
    pub refcli:i32,
    pub name: &'a str,
    pub address: &'a str,
    pub postal: &'a str,
    pub town: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputCustomer {
    pub refcli:i32,
    pub name: String,
    pub address: String,
    pub postal: String,
    pub town: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCustomer {
    #[serde(rename="customers")]
    pub customers: Vec<Customer>
}

impl Customer {
    pub fn get_by_id(pool: web::Data<Pool>, customer_id: i32) -> Result<Customer, diesel::result::Error> {
        let conn = pool.get().unwrap();
        customers::table.find(customer_id).get_result::<Customer>(&conn)
    }
    
    pub fn get(pool: web::Data<Pool>) -> Result<Vec<Customer>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let customers = customers::table.load::<Customer>(&conn)?;
        Ok(customers)
    }
    
    pub fn insert(
        db: web::Data<Pool>,
        cust: web::Json<InputCustomer>,
    ) -> Result<Customer, diesel::result::Error> {
        let conn = db.get().unwrap();
        let new_customer = NewCustomer {
            refcli:cust.refcli,
            name: &cust.name,
            address: &cust.address,
            postal: &cust.postal,
            town: &cust.town,
        };
        let res = diesel::insert_into(customers::table).values(&new_customer).get_result(&conn)?;
        Ok(res)
    }
    
    pub fn delete(db: web::Data<Pool>, customer_id: i32) -> Result<usize, diesel::result::Error> {
        let conn = db.get().unwrap();
        let count = diesel::delete(customers::table.find(customer_id)).execute(&conn)?;
        Ok(count)
    }
    
    pub fn update(
        db: web::Data<Pool>,
        customer_id: i32,
        item: web::Json<InputCustomer>) -> Result<Customer, diesel::result::Error> {
        let conn = db.get().unwrap();
        let updated_customer = NewCustomer {
            refcli:item.refcli,
            name: &item.name,
            address: &item.address,
            postal: &item.postal,
            town: &item.town,
        };
        let res = diesel::update(customers::table)
        .filter(customers::refcli.eq(customer_id))
        .set(&updated_customer)
        .get_result(&conn)?;
    
        Ok(res)
    }
}