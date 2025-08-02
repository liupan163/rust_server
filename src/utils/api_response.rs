use std::fmt::Display;

use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder, ResponseError};


#[derive(Debug)]
pub struct ApiResponse{
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode
}

impl ApiResponse{
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse{
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap()
        }
    }
}

impl Responder for ApiResponse{
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}

impl Display for ApiResponse{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Error: {} \n Status Code: {}",self.body,self.status_code)
    }
}

impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.status_code()).set_body(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_creation() {
        let response = ApiResponse::new(200, "Success".to_string());
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, "Success");
    }

    #[test]
    fn test_api_response_status_codes() {
        let response_400 = ApiResponse::new(400, "Bad Request".to_string());
        assert_eq!(response_400.status_code, 400);
        
        let response_404 = ApiResponse::new(404, "Not Found".to_string());
        assert_eq!(response_404.status_code, 404);
    }

    #[test]
    fn test_api_response_display() {
        let response = ApiResponse::new(404, "Resource not found".to_string());
        let display_string = format!("{}", response);
        assert!(display_string.contains("Error: Resource not found"));
        assert!(display_string.contains("Status Code: 404"));
    }
}