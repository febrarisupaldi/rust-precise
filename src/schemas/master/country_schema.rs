use chrono::NaiveDateTime;
use crate::utils::option::option_ts_seconds;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

use crate::{schemas::api_schemas::ApiResponse, settings::db_pool};
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]    
pub struct CountrySchema{
    pub country_id: u8,
    pub country_code: String,
    pub country_name: String,
    pub created_on: Option<NaiveDateTime>,
    pub created_by: String,

    #[serde(with = "option_ts_seconds")]
    pub updated_on: Option<NaiveDateTime>,
    pub updated_by: Option<String>
}

#[derive(Debug, Validate, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InsertCountrySchema{

    #[validate(length(min = 3, message = "country code must be at least 3 characters"))]
    pub country_code: String,

    #[validate(length(min = 3, message = "country name must be at least 3 characters"))]
    pub country_name: String,

    #[validate(required)]
    pub created_by: Option<String>
}

#[derive(Debug, Validate, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UpdateCountrySchema{

    #[validate(length(min = 3, message = "country code must be at least 3 characters"))]
    pub country_code: String,

    #[validate(length(min = 3, message = "country name must be at least 3 characters"))]
    pub country_name: String,

    #[validate(required)]
    pub updated_by: Option<String>,

    #[validate(required)]
    pub reason: Option<String>
}

#[derive(Deserialize, Serialize, FromRow, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct CountryNameQuery {
    pub country_name: String,
}

#[derive(Deserialize, Serialize, FromRow, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct CountryCodeQuery {
    pub country_code: String,
}

pub async fn validate_country_id(country_id: u8) -> Result<(), ValidationError> {
    let pool = db_pool().await;
    let data = sqlx::query_scalar::<_, i64>("select count(*) from precise.country where country_id=?")
        .bind(&country_id)
        .fetch_one(&pool)
        .await
        .map(|count| count > 0)
        .unwrap_or(false);

    if data{
        Ok(())
    } else {
        let mut err = ValidationError::new("country_id_not_found");
        err.message = Some("Country ID not found".into());
        Err(err)
    }
}

pub async fn validate_country_code(country_code: String) -> Result<(), ValidationError> {
    let pool = db_pool().await;
    let data = sqlx::query_scalar::<_, i64>("select count(*) from precise.country where country_code=?")
        .bind(&country_code)
        .fetch_one(&pool)
        .await
        .map(|count| count == 0)
        .unwrap_or(false);

    if data{
        Ok(())
    } else {
        let mut err = ValidationError::new("country_code_exists");
        err.message = Some("Country Code already exists".into());
        Err(err)
    }
}

#[utoipa::path(
    get,
    path = "/precise/api/master/countries",
    tag = "Country",
    responses(
        (status = 200, description="Countries data retrieved successfully", body = ApiResponse<CountrySchema>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_all_countries() {}

#[utoipa::path(
    post,
    tag = "Country",
    path = "/precise/api/master/countries",
    request_body= InsertCountrySchema,
    responses(
        (status = 200, description="Country created", body = ApiResponse<InsertCountrySchema>),
        (status = 400, description="Invalid Input", body = ApiResponse<String>),
        (status = 500, description="Failed to create city", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn create_country() {}

#[utoipa::path(
    get,
    tag = "Country",
    path = "/precise/api/master/countries/{country_id}",
    responses(
        (status = 200, description="Country updated", body = ApiResponse<CountrySchema>),
        (status = 404, description="City id does not exists", body = ApiResponse<String>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_country() {}


#[utoipa::path(
    put,
    tag = "Country",
    path = "/precise/api/master/countries/{country_id}",
    request_body = UpdateCountrySchema,
    responses(
        (status = 200, description="Country updated", body = ApiResponse<UpdateCountrySchema>),
        (status = 400, description="Invalid Input", body = ApiResponse<String>),
        (status = 404, description="City id does not exists", body = ApiResponse<String>),
        (status = 500, description="Failed to update city", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn update_country() {}

#[utoipa::path(
    get,
    tag = "Country",
    path = "/precise/api/master/countries/exists/code",
    params(CountryCodeQuery),
    responses(
        (status = 200, description="Country code exists", body = ApiResponse<String>),
        (status = 404, description="Country code does not exists", body = ApiResponse<String>),
        (status = 500, description="Failed to check country code", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn exists_country_code() {}

#[utoipa::path(
    get,
    tag = "Country",
    path = "/precise/api/master/countries/exists/name",
    params(CountryNameQuery),
    responses(
        (status = 200, description="Country name exists", body = ApiResponse<String>),
        (status = 404, description="Country name does not exist", body = ApiResponse<String>),
        (status = 500, description="Failed to check country code", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn exists_country_name() {}