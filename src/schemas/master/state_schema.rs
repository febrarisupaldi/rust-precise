use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::utils::option::option_ts_seconds;
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

use crate::{schemas::api_schemas::ApiResponse, settings::db_pool};
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StatesSchema{
    pub state_id: u8,
    pub state_code: String,
    pub state_name: String,
    pub country_id: u8,
    pub country_name: String,
    pub created_on: Option<NaiveDateTime>,
    pub created_by: String,
    
    #[serde(with = "option_ts_seconds")]
    pub updated_on: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate, ToSchema)]
pub struct InsertStatesSchema{
    #[validate(length(min = 3, message = "state code must be at least 3 characters"))]
    pub state_code: String,
    
    #[validate(length(min = 3, message = "state name must be at least 3 characters"))]
    pub state_name: String,
    
    pub country_id: u8,

    #[validate(required)]
    pub created_by: Option<String>
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate, ToSchema)]
pub struct UpdateStatesSchema{
    #[validate(length(min = 3, message = "state code must be at least 3 characters"))]
    pub state_code: String,
    
    #[validate(length(min = 3, message = "state name must be at least 3 characters"))]
    pub state_name: String,
    
    pub country_id: u8,
    pub updated_by: String
}

#[derive(Deserialize, Serialize, FromRow, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct StateNameQuery {
    pub state_name: String,
}

#[derive(Deserialize, Serialize, FromRow, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct StateCodeQuery {
    pub state_code: String,
}

pub async fn validate_state_id(state_id: u8) -> Result<(), ValidationError> {
    let pool = db_pool().await;
    let data = sqlx::query_scalar::<_, i64>("select count(*) from precise.state where state_id=?")
        .bind(&state_id)
        .fetch_one(&pool)
        .await
        .map(|count| count > 0)
        .unwrap_or(false);

    if data{
        Ok(())
    } else {
        let mut err = ValidationError::new("state_id_not_found");
        err.message = Some("State ID not found".into());
        Err(err)
    }
}

pub async fn validate_state_code(state_code: &str) -> Result<(), ValidationError> {
    let pool = db_pool().await;
    let data = sqlx::query_scalar::<_, i64>("select count(*) from precise.state where state_code=?")
        .bind(&state_code)
        .fetch_one(&pool)
        .await
        .map(|count| count == 0)
        .unwrap_or(false);

    if data{
        Ok(())
    } else {
        let mut err = ValidationError::new("state_code_exists");
        err.message = Some("State code already exists".into());
        Err(err)
    }
}

#[utoipa::path(
    get,
    tag = "State",
    path = "/precise/api/master/states",
    responses(
        (status = 200, description="states data retrieved successfully", body = ApiResponse<StatesSchema>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_all_states() {}

#[utoipa::path(
    post,
    tag = "State",
    path = "/precise/api/master/states",
    responses(
        (status = 200, description="states data retrieved successfully", body = ApiResponse<InsertStatesSchema>),
        (status = 400, description="Invalid Input", body = ApiResponse<String>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn create_state() {}

#[utoipa::path(
    get,
    tag= "State",
    path = "/precise/api/master/states/{state_id}",
    responses(
        (status = 200, description="states data retrieved successfully", body = ApiResponse<StatesSchema>),
        (status = 404, description="Data not found", body = ApiResponse<String>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_state() {}

#[utoipa::path(
    get,
    tag = "State",
    path = "/precise/api/master/states/{state_id}",
    responses(
        (status = 200, description="states data retrieved successfully", body = ApiResponse<StatesSchema>),
        (status = 404, description="Data not found", body = ApiResponse<String>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn update_state() {}

#[utoipa::path(
    get,
    tag = "State",
    path = "/precise/api/master/states/exists/code",
    params(StateCodeQuery),
    responses(
        (status = 200, description="State code exists", body = ApiResponse<String>),
        (status = 404, description="State code does not exists", body = ApiResponse<String>),
        (status = 500, description="Failed to check State code", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn exists_state_code() {}

#[utoipa::path(
    get,
    tag = "Country",
    path = "/precise/api/master/states/exists/name",
    params(StateNameQuery),
    responses(
        (status = 200, description="State name exists", body = ApiResponse<String>),
        (status = 404, description="State name does not exist", body = ApiResponse<String>),
        (status = 500, description="Failed to check State code", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn exists_state_name() {}