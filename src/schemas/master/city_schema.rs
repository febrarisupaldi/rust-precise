use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};
use crate::{helpers::option::option_ts_seconds, settings::db_pool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CitySchema{
    pub city_id: u8,
    pub city_code: String,
    pub city_name: String,
    pub state_id: u8
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CityNameQuery {
    pub city_name: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CityCodeQuery {
    pub city_code: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCitySchema{
    pub city_code: String,
    pub city_name: String,
    pub updated_by: Option<String>
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

// impl CitySchema{
//     pub fn new(city_id: i32, city_code: String, city_name: String, state_name: String, country_name: String, created_on: NaiveDateTime, created_by: String, updated_on: NaiveDateTime, updated_by: String) -> Self{
//         Self{
//             city_id,
//             city_code,
//             city_name,
//             state_name,
//             country_name,
//             created_on,
//             created_by,
//             updated_on,
//             updated_by
//         }
//     }

//     pub fn with_response(city: CitySchema) -> ApiResponse<CitySchema>{
//         ApiResponse::success("Data retrieved successfully", city)
//     }
// }