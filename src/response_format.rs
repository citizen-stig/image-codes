use actix_web::HttpRequest;

pub enum ResponseFormat {
    HTML,
    Image,
    Json,
}

pub fn get_response_format(req: HttpRequest) -> ResponseFormat {
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
