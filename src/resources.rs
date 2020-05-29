use core::fmt;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use base64::encode;
use serde::{Deserialize, Serialize};

use crate::barcode;
use crate::encoder::Encode;
use crate::qrcode;

#[derive(Deserialize, Debug)]
enum Encoding {
    BarCode,
    QRCode,
    Aztec,
    PDF417,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    encoding: Encoding,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
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

pub fn index(info: web::Path<Info>, query: web::Query<Params>) -> HttpResponse {
    let code = process_request(&info.encoding, query.payload.clone(), query.height);

    match code.output() {
        Ok(data) => {
            let result = encode(&data[..]);
            HttpResponse::build(StatusCode::OK)
                .content_type("text/html; charset=utf-8")
                .body(format!(
                    "<p>Welcome!</p><img src=\"data:image/png;base64, {}\"/>",
                    result
                ))
        }
        Err(error) => HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type("text/html; charset=utf-8")
            .body(format!("<p>Error!</p><p>{}<p/>", error)),
    }
}

#[cfg(test)]
mod tests {
    // use actix_web::dev::Payload;
    // use actix_web::test;
    // use actix_web::{FromRequest, HttpRequest};
    // use futures_util::stream::StreamExt;
    // use futures_util::stream::TryStreamExt;
    // use futures::stream::Stream;

    use super::*;

    // fn test_from_request(http_request: &HttpRequest) -> HttpResponse {
    //     let mut payload = Payload::None;
    //     let path: web::Path<Info> = web::Path::from_request(http_request, &mut payload).unwrap();
    //     let query: web::Query<Params> =
    //         web::Query::from_request(http_request, &mut payload).unwrap();
    //
    //     index(path, query)
    // }

    fn test_from_encoding(encoding: Encoding, payload: &str) -> HttpResponse {
        let info = Info { encoding };
        let path: web::Path<Info> = web::Path::from(info);

        let query_string = format!("payload={:?}", payload);
        let query: web::Query<Params> = web::Query::from_query(&query_string).unwrap();

        index(path, query)
    }

    #[test]
    fn barcode_ok() {
        let response = test_from_encoding(Encoding::BarCode, "aaa");
        assert_eq!(StatusCode::OK, response.status());
    }

    #[test]
    fn barcode_bad() {
        let response = test_from_encoding(Encoding::BarCode, "тест");
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }
}
