use core::fmt;

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use base64::encode;
use serde::{Deserialize, Serialize};

use crate::barcode;
use crate::encoder::Encode;
use crate::qrcode;
use std::panic;

#[derive(Deserialize, Debug)]
enum Encoding {
    BarCode,
    QRCode,
    Aztec,
    PDF417,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

enum ResponseFormat {
    HTML,
    Image,
    Json,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    encoding: Encoding,
}

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    payload: String,
    payload_type: String,
}

impl JsonResponse {
    pub fn new_from_data(data: Vec<u8>) -> Self {
        JsonResponse {
            payload: encode(&data[..]),
            payload_type: "image/png".to_string(),
        }
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

fn get_response_format(req: HttpRequest) -> ResponseFormat {
    let accept_header = req.headers().get("accept");
    if accept_header.is_none() {
        return ResponseFormat::HTML;
    }
    let accept_header = accept_header.unwrap();
    match accept_header.to_str() {
        Ok(accept_header) => {
            if accept_header.starts_with("image") {
                ResponseFormat::Image
            } else if accept_header.starts_with("application/json") {
                ResponseFormat::Json
            } else {
                ResponseFormat::HTML
            }
        }
        Err(_e) => ResponseFormat::HTML,
    }
}

fn get_successful_output(data: Vec<u8>, format: ResponseFormat) -> HttpResponse {
    match format {
        ResponseFormat::Image => HttpResponse::build(StatusCode::OK)
            .content_type("image/png")
            .body(data),
        ResponseFormat::HTML => {
            let result = encode(&data[..]);
            HttpResponse::build(StatusCode::OK)
                .content_type("text/html; charset=utf-8")
                .body(format!(
                    "<p>Welcome!</p><img src=\"data:image/png;base64, {}\"/>",
                    result
                ))
        }
        ResponseFormat::Json => {
            HttpResponse::build(StatusCode::OK).json(JsonResponse::new_from_data(data))
        }
    }
}

pub async fn index() -> Result<NamedFile, std::io::Error> {
    Ok(NamedFile::open("static/index.html")?)
}

pub async fn get_code(
    info: web::Path<Info>,
    query: web::Query<Params>,
    req: HttpRequest,
) -> HttpResponse {
    let process_result = panic::catch_unwind(|| {
        process_request(&info.encoding, query.payload.clone(), query.height)
    });

    if process_result.is_err() {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type("text/html; charset=utf-8")
            .body(format!("Encoding {:?} is not supported", info.encoding));
    }

    let code = process_result.unwrap();
    let response_format = get_response_format(req);
    match code.output() {
        Ok(data) => get_successful_output(data, response_format),
        Err(error) => HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type("text/html; charset=utf-8")
            .body(format!("<p>Error!</p><p>{}<p/>", error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_from_encoding(encoding: Encoding, payload: &str) -> HttpResponse {
        let info = Info { encoding };
        let path: web::Path<Info> = web::Path::from(info);

        let query_string = format!("payload={:?}", payload);
        let query: web::Query<Params> = web::Query::from_query(&query_string).unwrap();
        get_code(path, query)
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

    #[test]
    fn qrcode_ok() {
        let response = test_from_encoding(Encoding::QRCode, "тест");
        assert_eq!(StatusCode::OK, response.status());
    }

    #[test]
    fn pdf417_not_supported() {
        let response = test_from_encoding(Encoding::PDF417, "aaa");
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    #[test]
    fn aztec_not_supported() {
        let response = test_from_encoding(Encoding::Aztec, "aaa");
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }
}
