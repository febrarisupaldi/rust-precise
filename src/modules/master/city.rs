pub mod city{
    use actix_web::{web, Responder};
    use sqlx::MySqlPool;
    use validator::Validate;

    use crate::schemas::master::city_schema::validate_city_code;
    use crate::schemas::master::city_schema::{CitiesSchema, CityCodeQuery, CityNameQuery, CitySchema, InsertCitySchema, UpdateCitySchema};
    use crate::schemas::api_schemas::ApiResponse;
    use crate::schemas::master::state_schema::validate_state_id;
    #[utoipa::path(
        get,
        path = "/master/cities",
        responses(
            (status = 200, description = "Data retrieved successfully"),
            (status = 500, description = "Failed to fetch data")
        )
    )]
    pub async fn get_all_cities(pool:web::Data<MySqlPool>) -> impl Responder{
        let data = sqlx::query_as::<_, CitiesSchema>(
            "select city_id, city_code, city_name, state_name, country_name
            , c.created_on, c.created_by, c.updated_on, c.updated_by
            from precise.city c
            left join precise.state s on c.state_id = s.state_id
            left join precise.country co on s.country_id = co.country_id
            ")
            .fetch_all(pool.get_ref())
            .await;

        match data{
            Ok(result) => ApiResponse::success("Data retrieved successfully", result).to_http_response(),
            Err(err) => {
                println!("Error:{:?}", err);
                ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
            },
        }
    }

    pub async fn get_city((param, pool):(web::Path<String>, web::Data<MySqlPool>)) -> impl Responder{
        let data = sqlx::query_as::<_, CitySchema>(
            "select city_id, city_code, city_name, state_id from precise.city
            where city_id = ?
            ")
            .bind(param.into_inner())
            .fetch_one(pool.get_ref())
            .await;

        match data{
            Ok(result) => ApiResponse::success("Data retrieved successfully", result).to_http_response(),
            Err(_) => ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

    pub async fn insert_city((form, pool):(web::Json<InsertCitySchema>, web::Data<MySqlPool>)) -> impl Responder{
        let city_data = form.into_inner();

        if let Err(validation_errors) = city_data.validate(){
            return ApiResponse::<()>::error(400, &validation_errors.to_string()).to_http_response();
        }

        if let Err(validation_errors) = validate_state_id(city_data.state_id.unwrap()).await{
            return ApiResponse::<()>::error(400, &validation_errors.to_string()).to_http_response();
        }

        if let Err(validation_errors) = validate_city_code(&city_data.city_code).await{
            return ApiResponse::<()>::error(400, &validation_errors.to_string()).to_http_response();
        }

        let data = sqlx::query(
            "insert into precise.city(city_code, city_name, state_id, created_by)
            values(?, ?,?,?)
            ")
            .bind(city_data.city_code)
            .bind(city_data.city_name)
            .bind(city_data.state_id)
            .bind(city_data.created_by)
            .execute(pool.get_ref())
            .await;

        match data{
            Ok(result) => ApiResponse::success("Data inserted successfully", result.last_insert_id()).to_http_response(),
            Err(_) => ApiResponse::<()>::error(500, "Failed to insert data").to_http_response()
        }
    }

    pub async fn update_city((param, form, pool):(web::Path<String>, web::Json<UpdateCitySchema>, web::Data<MySqlPool>)) -> impl Responder{
        let city = form.into_inner();
        let data = sqlx::query(
            "update precise.city set city_code = ?, city_name = ?, updated_by = ?
            where city_id = ?
            ")
            .bind(city.city_code)
            .bind(city.city_name)
            .bind(city.updated_by)
            .bind(param.into_inner())
            .execute(pool.get_ref())
            .await;

        match data{
            Ok(result) => {
                if result.rows_affected() == 0 {
                    ApiResponse::<()>::error(404, "Data not found").to_http_response()
                } else {
                    ApiResponse::success("Data updated successfully", result.last_insert_id()).to_http_response()
                }
            },
            Err(_) => ApiResponse::<()>::error(500, "Failed to update data").to_http_response()
        }
    }

    pub async fn exists_cities_name((query, pool):(web::Query<CityNameQuery>, web::Data<MySqlPool>)) -> impl Responder{
        let data = sqlx::query!(
            "select exists (select 1 from precise.city where city_name = ?) as `exists`", query.city_name)
            .fetch_one(pool.get_ref())
            .await;

        match data{
            Ok(result) => {
                if result.exists == 1 {
                    ApiResponse::success("Data exists", true).to_http_response()
                } else {
                    ApiResponse::success("Data not exists", false).to_http_response()
                }
            },
            Err(_) => ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

    pub async fn exists_cities_code((query, pool):(web::Query<CityCodeQuery>, web::Data<MySqlPool>)) -> impl Responder{
        let data = sqlx::query!(
            "select exists (select 1 from precise.city where city_code = ?) as `exists`", query.city_code)
            .fetch_one(pool.get_ref())
            .await;

        match data{
            Ok(result) => {
                if result.exists == 1 {
                    ApiResponse::success("Data exists", true).to_http_response()
                } else {
                    ApiResponse::success("Data not exists", false).to_http_response()
                }
            },
            Err(_) => ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

}