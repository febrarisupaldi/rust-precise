use actix_web::{http::StatusCode, web, HttpResponse};
use serde_json::Error;
use sqlx::{MySqlPool, Row};
use validator::Validate;
use crate::schemas::UserLogin;
use bcrypt::{verify};


pub async fn login((form, pool):(web::Json<UserLogin>, web::Data<MySqlPool>)) -> Result<HttpResponse, Error>{
    let user = form.into_inner();

    let validation = user.validate();

    if validation.is_err(){
        let err = user.validate().err().unwrap();
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(serde_json::json!({"error": err})));
    }

    let check = sqlx::query("select password from precise.users where user_id = ?")
        .bind(user.user_id)
        .fetch_one(pool.get_ref())
        .await;

    match check{
        Ok(row) =>{
            let db_pass = row.get("password");

            if verify(&user.password.unwrap(), db_pass).is_ok(){
                let response = serde_json::json!({"message":"Success Login"});
                Ok(HttpResponse::Ok().status(StatusCode::OK).json(response))
            }else{
                let response = serde_json::json!({"message":"Success Login"});
                Ok(HttpResponse::Unauthorized().status(StatusCode::UNAUTHORIZED).json(response))
            }

        }
        Err(_) =>{
            Ok(HttpResponse::NotFound().status(StatusCode::NOT_FOUND).json(serde_json::json!({"message":"No found record"})))
        }
    }
}
