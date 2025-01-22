use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct UserLogin{
    #[validate(required(message="username is required"))]
    pub user_id: Option<String>,
    #[validate(required(message="password is required"))]
    pub password: Option<String>
}