use crate::student::{Student,Course};
use actix_web::{delete,get,post,web, Error};
use actix_storage::{Storage};

#[post("/students")]
async fn create(
    item:web::Json<Student>,
    storage: Storage,
) -> Result<web::Json<Student>, Error> {

    let mut courses:Vec<Course> = Vec::new();

    for item in &item.courses {
        let course = Course {
            classroom:item.classroom.to_string(),
            course:item.course.to_string(),
        };
        courses.push(course);
    }
    let out = Student {
        ident: item.ident.to_string(),
        firstname: item.firstname.to_string(),
        lastname: item.lastname.to_string(),
        courses:courses,
    };

    // Setting back the data to storage
    storage.set(&item.ident.to_string(), &out).await?;

    Ok(web::Json(out))
}
#[get("/students/{ident}")]
async fn retrieve(
    ident:web::Path<String>,
    storage: Storage,
) -> Result<web::Json<Student>, Error> {

    let student = if let Some(student) = storage.get::<_, Student>(&ident.to_string()).await? {
        student
    } else {
        Student::default()
    };

    Ok(web::Json(student))
}

#[delete("/students/{ident}")]
async fn remove(
    ident:web::Path<String>,
    storage: Storage,
) -> Result<String, Error> {
    storage.delete(&ident.to_string()).await.unwrap_or_default();
    Ok(String::from("deleted"))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(retrieve);
    cfg.service(create);
    cfg.service(remove);
}