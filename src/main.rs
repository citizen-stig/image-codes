use std::fmt;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

mod encoders;

#[derive(Deserialize, Debug)]
enum Encoding {
    Code128,
    QRCode,
    Aztec,
    PDF417,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize)]
struct InputMessage {
    payload: String,
    encoding: Encoding,
}

#[derive(Deserialize, Serialize)]
struct OutputMessage {
    data: String,
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn encode_message(message: web::Json<InputMessage>) -> Result<HttpResponse> {
    let data = match message.encoding {
        Encoding::Code128 => encoders::encode_barcode128(&message.payload),
        Encoding::QRCode => encoders::encode_qrcode(&message.payload),
        _ => String::from("NOT SUPPORTED YET"),
    };
    Ok(HttpResponse::Ok().json(OutputMessage { data }))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/encode", web::post().to(encode_message))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
