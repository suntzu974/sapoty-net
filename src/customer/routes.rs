use crate::customer::{Customer,InputCustomer};
use actix_web::{post,web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use actix_multipart::Multipart;
use crate::Pool;
use std::fs;
use std::io::Write;
use calamine::{open_workbook, Xlsx, Reader };
use askama::Template;
use crate::diesel::RunQueryDsl;
use crate::schema::customers;


#[derive(Template)]
#[template(path = "customers.html")] 
struct CustomerlistTemplate<'a> {
    title: &'a str,
    customers: &'a Vec<Customer>,
}



#[post("/excel")]
pub async fn upload_excel(db: web::Data<Pool>,mut payload: Multipart) -> Result<HttpResponse, Error> {
    let documents_files = std::env::var("DOCUMENTS_FILES").expect("DOWNLOAD FILES DIRECTORY NOT FOUND");
    let mut filename = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        filename = format!("{}", content_type.get_filename().unwrap(), );
        let filepath = format!("{}/{}", documents_files, sanitize_filename::sanitize(&filename));
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }
    }
    
    let urlpath = format!("{}/{}",documents_files,sanitize_filename::sanitize(&filename));
    let template = CustomerlistTemplate {
        title: &urlpath,
        customers: &extract_data(db,urlpath.clone()),
    };
    let res = template
        .render()
        .map_err(|_| HttpResponse::InternalServerError())?;
        fs::remove_file(urlpath.clone()).unwrap();

        Ok(HttpResponse::Ok().content_type("text/html").body(res))


}

pub fn extract_data(db: web::Data<Pool>,url_path: String) -> Vec<Customer> {
    let conn = db.get().unwrap();
    let mut clients :Vec<Customer> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(url_path).unwrap();
    if let Some(Ok(range)) = workbook.worksheet_range("clients")
    {
            range.rows().into_iter().skip(1).for_each(|row| {
                let customer = Customer {
                    refcli: row[0].to_string().parse::<i32>().unwrap(),
                    name:row[1].to_string(),
                    address:row[2].to_string(),
                    postal:row[3].to_string(),
                    town:row[4].to_string()
                };
            // body is loaded, now we can deserialize serde-json
            let res:Customer = diesel::insert_into(customers::table).values(&customer).get_result(&conn).unwrap();
            clients.push(res);
            });
            return clients;
    }
    return clients;
}
#[post("/customers")]
pub async fn create(
    db: web::Data<Pool>,
    item: web::Json<InputCustomer>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Customer::insert(db, item))
        .await
        .map(|customer| HttpResponse::Created().json(customer))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(upload_excel);
    cfg.service(create);
 }