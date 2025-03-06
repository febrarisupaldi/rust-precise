use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};
use crate::{utils::option::option_ts_seconds, settings::db_pool};
use crate::schemas::api_schemas::ApiResponse;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CitiesSchema{
    pub city_id: u8,
    pub city_code: String,
    pub city_name: String,
    pub state_name: String,
    pub country_name: String,
    pub created_on: Option<NaiveDateTime>,
    pub created_by: String,

    #[serde(with = "option_ts_seconds")]
    pub updated_on: Option<NaiveDateTime>,

    pub updated_by: Option<String>
}
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CitySchema{
    pub city_id: u8,
    pub city_code: String,
    pub city_name: String,
    pub state_id: u8
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct InsertCitySchema{

    #[validate(length(min = 3, message = "city code must be at least 3 characters"))]
    pub city_code: String,

    #[validate(length(min = 3, message = "city name must be at least 3 characters"))]
    pub city_name: String,

    #[validate(required)]
    pub state_id: Option<u8>,

    #[validate(required)]
    pub created_by: Option<String>
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, Validate)]
pub struct UpdateCitySchema{
    pub city_code: String,
    pub city_name: String,
    pub updated_by: Option<String>
}


#[derive(Deserialize, Serialize, FromRow, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CityNameQuery {
    pub city_name: String,
}

#[derive(Deserialize, Serialize, FromRow, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CityCodeQuery {
    pub city_code: String,
}

pub async fn validate_city_code(city_code: &str) -> Result<(), ValidationError> {
    let pool = db_pool().await;
    let data = sqlx::query_scalar::<_, i64>("select count(*) from precise.city where city_code=?")
        .bind(city_code)
        .fetch_one(&pool)
        .await
        .map(|count| count > 1)
        .unwrap_or(false);

    if data{
        Ok(())
    } else {
        let mut err = ValidationError::new("city_code_exists");
        err.message = Some("City Code already exists".into());
        Err(err)
    }
}

#[utoipa::path(
    get,
    tag = "City",
    path = "/master/cities",
    responses(
        (status = 200, description = "Data retrieved successfully", body = ApiResponse<CitiesSchema>),
        (status = 500, description = "Failed to fetch data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_all_cities() {}

#[utoipa::path(
    get,
    tag = "City",
    path = "/master/cities/{id}",
    responses(
        (status = 200, description = "Data retrieved successfully", body = ApiResponse<CitiesSchema>),
        (status = 500, description = "Failed to fetch data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_city() {}

#[utoipa::path(
    post,
    tag = "City",
    path = "/master/cities",
    request_body = InsertCitySchema,
    responses(
        (status = 200, description = "Data retrieved successfully", body = ApiResponse<InsertCitySchema>),
        (status = 400, description = "Invalid Input", body = ApiResponse<String>),
        (status = 500, description = "Failed to fetch data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn create_city() {}

#[utoipa::path(
    put,
    tag = "City",
    path = "/master/cities/{id}",
    request_body = UpdateCitySchema,
    responses(
        (status = 200, description = "Data retrieved successfully", body = ApiResponse<UpdateCitySchema>),
        (status = 400, description = "Invalid input", body = ApiResponse<String>),
        (status = 404, description = "Data not found", body = ApiResponse<String>),
        (status = 500, description = "Failed to fetch data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn update_city() {}

#[utoipa::path(
    get,
    tag = "City",
    path = "/precise/api/master/cities/exists/code",
    params(CityCodeQuery),
    responses(
        (status = 200, description="City code exists", body = ApiResponse<u8>),
        (status = 500, description="Failed to check city code", body = ApiResponse<u8>)
    ),
    security(("bearer_auth" = []))
)]
pub fn exists_city_code() {}

#[utoipa::path(
    get,
    tag = "Country",
    path = "/precise/api/master/cities/exists/name",
    params(CityNameQuery),
    responses(
        (status = 200, description="City name exists", body = ApiResponse<u8>),
        (status = 500, description="Failed to check city code", body = ApiResponse<u8>)
    ),
    security(("bearer_auth" = []))
)]
pub fn exists_city_name() {}