use std::fmt;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::Encoding::BarCode;
use crate::encoder::Encoder;
use base64::encode;

mod core;
mod encoder;
mod barcode;
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
    encoding: Encoding
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

#[derive(Deserialize, Serialize, Debug)]
enum ImageOutputFormat {
    PNG,
    JPEG,
    GIF,
}


impl Default for ImageOutputFormat {
    fn default() -> Self {
        ImageOutputFormat::PNG
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Params {
    payload: String,

    #[serde(default)]
    image_type: ImageOutputFormat,
}


fn index(info: web::Path<Info>, query: web::Query<Params>) -> HttpResponse {
    let height = 300;
    let data = match info.encoding {
        Encoding::BarCode => {
            let code = barcode::BarCode::new(query.payload.clone());
            code.output()
        },
        Encoding::QRCode => {
            let code = qrcode::QRCode::new(query.payload.clone());
            code.output()
        },
        _ => panic!("Not supported yet!!!"),
    };

    let result = encode(&data[..]);

    // response
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!("<p>Welcome!</p><img src=\"data:image/png;base64, {}\"/>", result))
}




fn main() {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/encode/{encoding}").to(index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
