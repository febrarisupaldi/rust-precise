pub mod state{
    use actix_web::{web, Responder};
    use serde_json::json;
    use sqlx::{MySqlPool, Transaction};

    use crate::{modules::helper::reason::reason::{update_reason, KindTransaction}, schemas::{api_schemas::ApiResponse, master::state_schema::{InsertStatesSchema, StateCodeQuery, StateNameQuery, StatesSchema, UpdateStatesSchema}}};

    pub async fn get_all_states(pool: web::Data<MySqlPool>) -> impl Responder{
        let data = sqlx::query_as::<_, StatesSchema>("select 
            pState.state_id, 
            pState.state_code, 
            pState.state_name, 
            pState.country_id, 
            pCountry.country_name, 
            pState.created_on, 
            pState.created_by, 
            pState.updated_on, 
            pState.updated_by
                from
            precise.state as pState
                left join precise.country pCountry on
                pState.country_id = pCountry.country_id")
            .fetch_all(pool.get_ref())
            .await;

        match data{
            Ok(result) => {
                if result.len() > 0 {
                    ApiResponse::success("Data found", result).to_http_response()
                } else {
                    ApiResponse::success("Data not found", result).to_http_response()
                }
            },
            Err(_) => ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

    pub async fn get_state((state_id, pool): (web::Path<u8>, web::Data<MySqlPool>)) -> impl Responder{
        let state = state_id.into_inner();
        let data = sqlx::query_as::<_, StatesSchema>("select 
            pState.state_id, 
            pState.state_code, 
            pState.state_name, 
            pState.country_id, 
            pCountry.country_name, 
            pState.created_on, 
            pState.created_by, 
            pState.updated_on, 
            pState.updated_by
                from
            precise.state as pState
                left join precise.country pCountry on
                pState.country_id = pCountry.country_id
                where pState.state_id = ?")
            .bind(&state)
            .fetch_one(pool.get_ref())
            .await;

        match data{
            Ok(result) => ApiResponse::success("Data found", result).to_http_response(),
            Err(_) => ApiResponse::<()>::error(500, "Failed to fetch data").to_http_response()
        }
    }

    pub async fn create_state((pool, state): (web::Data<MySqlPool>, web::Json<InsertStatesSchema>)) -> impl Responder{
        let data = sqlx::query("insert into precise.state (state_code, state_name, country_id, created_by) values (?, ?, ?, ?)")
            .bind(&state.state_code)
            .bind(&state.state_name)
            .bind(&state.country_id)
            .bind(&state.created_by)
            .execute(pool.get_ref())
            .await;

        match data{
            Ok(_) => ApiResponse::success("Data inserted", state).to_http_response(),
            Err(_) => ApiResponse::<()>::error(500, "Failed to insert data").to_http_response()
        }
    }

    pub async fn update_state((pool, state_id, state): (web::Data<MySqlPool>, web::Path<u8>, web::Json<UpdateStatesSchema>)) -> impl Responder{
        let state_data = state.into_inner();

        let mut transaction: Transaction<'_, _> = match pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => return ApiResponse::<()>::error(500, "Failed to update data").to_http_response()
        };

        let data_reason = json!(state_data);

        if let Err(_) = update_reason((KindTransaction::Update, &data_reason, pool)).await {
            let _ = transaction.rollback().await;
            return ApiResponse::<()>::error(500, "Failed to update data").to_http_response();
        }

        let state_result = sqlx::query("update precise.state set state_code = ?, state_name = ?, country_id = ?, updated_by = ? where state_id = ?")
            .bind(&state_data.state_code)
            .bind(&state_data.state_name)
            .bind(&state_data.country_id)
            .bind(&state_data.updated_by)
            .bind(&state_id.into_inner())
            .execute(&mut *transaction)
            .await;

        if state_result.is_err() {
            let _ = transaction.rollback().await;
            return ApiResponse::<()>::error(500, "Failed to update data").to_http_response();
        } else {
            if let Err(_) = transaction.commit().await {
                return ApiResponse::<()>::error(500, "Failed to commit transaction").to_http_response();
            }
            ApiResponse::success("Data updated successfully", state_data).to_http_response()
        }
    }

    pub async fn exists_state_name((query, pool):(web::Query<StateNameQuery>, web::Data<MySqlPool>)) -> impl Responder{
        let data = sqlx::query!(
            "select exists (select 1 from precise.state where state_name = ?) as `exists`", query.state_name)
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

    pub async fn exists_state_code((query, pool):(web::Query<StateCodeQuery>, web::Data<MySqlPool>)) -> impl Responder{
        let data = sqlx::query!(
            "select exists (select 1 from precise.state where state_code = ?) as `exists`", query.state_code)
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