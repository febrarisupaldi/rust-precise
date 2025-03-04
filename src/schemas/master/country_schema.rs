use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::schemas::api_schemas::ApiResponse;
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]    
pub struct CountrySchema{
    pub country_id: u8,
    pub country_code: String,
    pub country_name: String,
    pub created_on: Option<NaiveDateTime>,
    pub created_by: String,
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



#[utoipa::path(
    get,
    path = "/precise/api/master/countries",
    responses(
        (status = 200, description="Countries data retrieved successfully", body = ApiResponse<CountrySchema>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_all_countries() {}

#[utoipa::path(
    post,
    path = "/precise/api/master/countries",
    responses(
        (status = 201, description="Country created", body = ApiResponse<CountrySchema>),
        (status = 400, description="Invalid Input", body = ApiResponse<String>),
        (status = 500, description="Failed to create city", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn create_country() {}

#[utoipa::path(
    get,
    path = "/precise/api/master/countries/{country_id}",
    responses(
        (status = 200, description="Country updated", body = ApiResponse<CountrySchema>),
        (status = 500, description="Failed to retrieve data", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn get_country() {}


#[utoipa::path(
    put,
    path = "/precise/api/master/countries/{country_id}",
    responses(
        (status = 200, description="Country updated", body = ApiResponse<UpdateCountrySchema>),
        (status = 400, description="Invalid Input", body = ApiResponse<String>),
        (status = 500, description="Failed to update city", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn update_country() {}

#[utoipa::path(
    get,
    path = "/precise/api/master/countries/exists/code",
    params(CountryCodeQuery),
    responses(
        (status = 200, description="Country code exists", body = ApiResponse<String>),
        (status = 404, description="Country code does not exist", body = ApiResponse<String>),
        (status = 500, description="Failed to check country code", body = ApiResponse<String>)
    ),
    security(("bearer_auth" = []))
)]
pub fn exists_country_code() {}

#[utoipa::path(
    get,
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