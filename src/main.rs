use std::fmt;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

mod core;

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
    let data = match info.encoding {
        Encoding::BarCode => core::encode_barcode128(&query.payload.to_uppercase(), 300),
        Encoding::QRCode => core::encode_qrcode(&query.payload, 500),
        _ => "NOT SUPPORTED YET".to_owned(),
    };

    // response
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!("<p>Welcome!</p><img src=\"data:image/png;base64, {}\"/>", &data))
}

//fn encode_message(message: web::Json<InputMessage>) -> Result<HttpResponse> {
//    let data = match message.encoding {
//        Encoding::Code128 => core::encode_barcode128(&message.payload, 300),
//        Encoding::QRCode => core::encode_qrcode(&message.payload, 300),
//        _ => String::from("NOT SUPPORTED YET"),
//    };
//    Ok(HttpResponse::Ok().json(OutputMessage { data }))
//}


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
