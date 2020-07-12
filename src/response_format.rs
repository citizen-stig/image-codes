use actix_web::HttpRequest;

#[derive(Debug)]
pub enum ResponseFormat {
    HTML,
    Image,
    JSON,
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
                ResponseFormat::JSON
            } else {
                ResponseFormat::HTML
            }
        }
        Err(_e) => ResponseFormat::HTML,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    fn test_header(header: &str) -> ResponseFormat {
        let req = test::TestRequest::with_header("accept", header).to_http_request();
        get_response_format(req)
    }

    #[test]
    fn test_no_header() {
        let req = test::TestRequest::with_header("agent", "test").to_http_request();
        assert!(matches!(get_response_format(req), ResponseFormat::HTML));
    }

    #[test]
    fn test_html_header() {
        assert!(matches!(test_header("text/html"), ResponseFormat::HTML));
    }

    #[test]
    fn test_image() {
        assert!(matches!(test_header("image/png"), ResponseFormat::Image));
    }

    #[test]
    fn test_json() {
        assert!(matches!(
            test_header("application/json"),
            ResponseFormat::JSON
        ));
    }

    #[test]
    fn test_accept_title_case() {
        let req = test::TestRequest::with_header("Accept", "image/webp").to_http_request();
        assert!(matches!(get_response_format(req), ResponseFormat::Image));
    }
}
