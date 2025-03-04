use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ReasonRequestSchema{
    #[validate(length(min = 1, message = "You should provide a reason"))]
    pub reason : String,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>
}