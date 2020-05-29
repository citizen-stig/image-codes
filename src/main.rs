use actix_web::{web, App, HttpServer};

mod barcode;
mod encoder;
mod qrcode;
mod resources;

fn main() {
    // TODO: Params for bind address
    HttpServer::new(|| {
        App::new().service(web::resource("/encode/{encoding}").to(resources::index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
