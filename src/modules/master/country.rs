pub mod country
{
    use actix_web::{web, HttpResponse, Responder};
    use serde_json::json;
    use sqlx::{MySqlPool, Transaction};
    use validator::Validate;

    use crate::modules::helper::reason::reason::{update_reason, KindTransaction};
    use crate::schemas::master::country_schema::{CountryCodeQuery, CountryNameQuery, CountrySchema, InsertCountrySchema, UpdateCountrySchema};
    use crate::schemas::api_schemas::ApiResponse;

    pub async fn get_all_countries(pool: web::Data<MySqlPool>) -> HttpResponse {
        let data = sqlx::query_as::<_, CountrySchema>("select country_id, country_code, country_name, created_on, created_by, updated_on, updated_by from precise.country")
            .fetch_all(pool.get_ref())
            .await;

        match data {
            Ok(result) => ApiResponse::success("Data retrieved successfully", result).to_http_response(),
            Err(_) =>
                ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

    pub async fn get_country((path, pool): (web::Path<String>, web::Data<MySqlPool>)) -> HttpResponse {
        let data = sqlx::query_as::<_, CountrySchema>("select country_id, country_code, country_name, created_on, created_by, updated_on, updated_by from precise.country where country_id = ?")
            .bind(path.into_inner())
            .fetch_one(pool.get_ref())
            .await;

        match data {
            Ok(result) => ApiResponse::success("Data retrieved successfully", result).to_http_response(),
            Err(_) =>
                ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

    pub async fn create_country((form, pool): (web::Json<InsertCountrySchema>, web::Data<MySqlPool>)) -> HttpResponse {
        let country_data = form.into_inner();

        if let Err(validation_errors) = country_data.validate(){
            return ApiResponse::<()>::error(400, &validation_errors.to_string()).to_http_response();
        }

        let data = sqlx::query(
            "insert into precise.country(country_code, country_name, created_by)
            values(?, ?, ?)
            ")
            .bind(country_data.country_code.clone())
            .bind(country_data.country_name.clone())
            .bind(country_data.created_by.clone())
            .execute(pool.get_ref())
            .await;

        match data {
            Ok(_) => ApiResponse::success("Data inserted successfully", country_data).to_http_response(),
            Err(_) => ApiResponse::<()>::error(500, "Failed to insert data").to_http_response()
        }
    }

    pub async fn update_country((param, form, pool): (web::Path<String>, web::Json<UpdateCountrySchema>, web::Data<MySqlPool>)) -> HttpResponse {
        let country_data = form.into_inner();
        
        let mut transaction: Transaction<'_, _> = match pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => return ApiResponse::<()>::error(500, "Failed to update data").to_http_response()
        };

        let data_reason = json!(country_data);

        if let Err(_) = update_reason((KindTransaction::Update, &data_reason, pool)).await {
            let _ = transaction.rollback().await;
            return ApiResponse::<()>::error(500, "Failed to update data").to_http_response();
        }
        
        let country_result = sqlx::query(
            "update precise.country set country_code = ?, country_name = ?, updated_by = ?
            where country_id = ?
            ")
            .bind(country_data.country_code.clone())
            .bind(country_data.country_name.clone())
            .bind(country_data.updated_by.clone())
            .bind(param.into_inner())
            .execute(&mut *transaction)
            .await;

        if country_result.is_err() {
            let _ = transaction.rollback().await;
            return ApiResponse::<()>::error(500, "Failed to update data").to_http_response();
        } else {
            if let Err(_) = transaction.commit().await {
                return ApiResponse::<()>::error(500, "Failed to commit transaction").to_http_response();
            }
            ApiResponse::success("Data updated successfully", country_data).to_http_response()
        }
    }

    
    pub async fn exists_country_name((query, pool):(web::Query<CountryNameQuery>, web::Data<MySqlPool>)) -> impl Responder{
        let data = sqlx::query!(
            "select exists (select 1 from precise.country where country_name = ?) as `exists`", query.country_name)
            .fetch_one(pool.get_ref())
            .await;

        match data{
            Ok(result) => {
                if result.exists == 1 {
                    ApiResponse::success("Data exists", result.exists).to_http_response()
                } else {
                    ApiResponse::success("Data not exists", result.exists).to_http_response()
                }
            },
            Err(_) => ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

    pub async fn exists_country_code((query, pool):(web::Query<CountryCodeQuery>, web::Data<MySqlPool>)) -> impl Responder{
        let data = sqlx::query!(
            "select exists (select 1 from precise.country where country_code = ?) as `exists`", query.country_code)
            .fetch_one(pool.get_ref())
            .await;

        match data{
            Ok(result) => {
                if result.exists == 1 {
                    ApiResponse::success("Data exists", result.exists).to_http_response()
                } else {
                    ApiResponse::success("Data not exists", result.exists).to_http_response()
                }
            },
            Err(_) => ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }
}