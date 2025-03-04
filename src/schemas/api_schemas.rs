use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T>{
    pub status: u16,
    pub message: String,
    pub data: Option<T>
}

impl <T: Serialize> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> Self{
        ApiResponse{
            status: 200,
            message: message.to_string(),
            data: Some(data)
        }
    }

    pub fn error(status: u16, message: &str) -> Self{
        ApiResponse{
            status,
            message: message.to_string(),
            data: None
        }
    }

    pub fn to_http_response(self) -> HttpResponse{
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(self)
    }
}