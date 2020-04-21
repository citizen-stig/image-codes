use std::fmt;

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer};
use base64::encode;
use serde::{Deserialize, Serialize};

use crate::encoder::Encode;

mod barcode;
mod encoder;
mod qrcode;

#[derive(Deserialize, Debug)]
enum Encoding {
    BarCode,
    QRCode,
    Aztec,
    PDF417,
}

#[derive(Deserialize, Debug)]
struct Info {
    encoding: Encoding,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Params {
    payload: String,
    #[serde(default = "default_height")]
    height: u32,
}

fn default_height() -> u32 {
    300
}

fn process_request(encoding: &Encoding, payload: String, height: u32) -> Box<dyn Encode> {
    match encoding {
        Encoding::BarCode => Box::new(barcode::BarCode::new(payload, height)),
        Encoding::QRCode => Box::new(qrcode::QRCode::new(payload, height)),
        _ => panic!("Not supported yet!!!"),
    }
}

fn index(info: web::Path<Info>, query: web::Query<Params>) -> HttpResponse {
    let code = process_request(&info.encoding, query.payload.clone(), query.height);
    let data = code.output();

    let result = encode(&data[..]);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "<p>Welcome!</p><img src=\"data:image/png;base64, {}\"/>",
            result
        ))
}

fn main() {
    HttpServer::new(|| App::new().service(web::resource("/encode/{encoding}").to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
