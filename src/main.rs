use actix_web::{web, App, HttpServer};

mod encoders;
mod resources;
mod response_format;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // TODO: Params for bind address
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(resources::index))
            .route("/encode/{encoding}", web::get().to(resources::get_code))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
