extern crate image;
use image::ImageFormat;
pub async fn resize_for_thumbnail(filename: String) -> String {
    let pictures_originals = std::env::var("PICTURES_ORIGINALS").expect("PICTURES DIRECTORY NOT FOUND");
    let pictures_thumbnails = std::env::var("PICTURES_THUMBNAILS").expect("PICTURES DIRECTORY NOT FOUND");

    let host = std::env::var("HOST").expect("HOST NOT FOUND");
    let port = std::env::var("PORT").expect("HOST NOT FOUND");

    let mut filepath = format!("{}/{}", pictures_originals, sanitize_filename::sanitize(&filename));

    let img = image::open(filepath).unwrap();
    let scaled = img.thumbnail(400, 400);


    filepath = format!("{}/{}", pictures_thumbnails, sanitize_filename::sanitize(&filename));
    let smallpicture = format!("{}", sanitize_filename::sanitize(&filename));
    let mut output = std::fs::File::create(filepath.clone()).unwrap();
    scaled.write_to(&mut output, ImageFormat::Jpeg).unwrap();
    let urlpath = format!("{}://{}:{}/thumbnail/{}","https",host,port,smallpicture,);
    return urlpath.clone();
}
pub async fn resize_for_web(filename: String) -> String {
    let pictures_originals = std::env::var("PICTURES_ORIGINALS").expect("PICTURES DIRECTORY NOT FOUND");
    let pictures_web = std::env::var("PICTURES_WEB").expect("PICTURES DIRECTORY NOT FOUND");

    let host = std::env::var("HOST").expect("HOST NOT FOUND");
    let port = std::env::var("PORT").expect("HOST NOT FOUND");

    let mut filepath = format!("{}/{}", pictures_originals, sanitize_filename::sanitize(&filename));

    let img = image::open(filepath).unwrap();
    let scaled = img.thumbnail(1024, 768);


    filepath = format!("{}/{}", pictures_web, sanitize_filename::sanitize(&filename));
    let smallpicture = format!("{}", sanitize_filename::sanitize(&filename));
    let mut output = std::fs::File::create(filepath.clone()).unwrap();
    scaled.write_to(&mut output, ImageFormat::Jpeg).unwrap();
    let urlpath = format!("{}://{}:{}/web/{}","https",host,port,smallpicture,);
    return urlpath.clone();
}