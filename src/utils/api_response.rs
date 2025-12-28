use actix_web::http::StatusCode;

#[derive(serde::Serialize, Debug)]
pub struct ApiError {
    pub error: String,
    pub status_code: u16
}

impl ApiError {
    pub fn new(error: String, status_code: StatusCode) -> Self {
        if std::env::var("RUST_LOG").unwrap_or("INFO".to_string()).to_lowercase().contains("debug") {
            eprintln!("[ERROR] {}", error);
        }

        Self { error, status_code: status_code.as_u16() }
    }
}

#[derive(serde::Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

impl ApiErrorResponse {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

#[derive(serde::Serialize)]
pub struct ApiSuccess<T> {
    pub data: T,
    pub status_code: u16
}

impl<T> ApiSuccess<T> {
    pub fn new(data: T, status_code: StatusCode) -> Self {
        Self { data, status_code: status_code.as_u16() }
    }
}