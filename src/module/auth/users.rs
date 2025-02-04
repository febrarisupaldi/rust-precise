use std::env;

use actix_web::{http::StatusCode, web, HttpResponse};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::Error;
use sqlx::{MySqlPool, Row};
use validator::Validate;
use crate::{middleware::Claims, schemas::UserLogin};
use bcrypt::{verify};


async fn create_jwt(user_id: &str) -> String{
    let secret = env::var("JWT_SECRET").expect("No secret key in environment");
    let expiration = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::seconds(480*60))
    .expect("Valid timestamp")
    .timestamp() as usize;

    let claims = Claims{
        sub: user_id.to_owned(),
        exp: expiration
    };

    encode(&Header::new(Algorithm::default()), &claims, &EncodingKey::from_secret(secret.as_ref())).expect("Token cannot be created")
}

pub async fn login((form, pool):(web::Json<UserLogin>, web::Data<MySqlPool>)) -> Result<HttpResponse, Error>{
    let user = form.into_inner();

    let validation = user.validate();

    if validation.is_err(){
        let err = user.validate().err().unwrap();
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(serde_json::json!({"status":"error","message": err})));
    }

    let check = sqlx::query("select password from precise.users where user_id = ?")
        .bind(user.user_id.clone())
        .fetch_one(pool.get_ref())
        .await;

    match check{
        Ok(row) =>{
            let db_pass = row.get("password");

            if verify(&user.password.unwrap(), db_pass).is_ok(){
                let token = create_jwt(user.user_id.clone().unwrap().as_str()).await;
                let response = serde_json::json!({"status":"ok","message":"Success Login", "token": token});
                Ok(HttpResponse::Ok().status(StatusCode::OK).json(response))
            }else{
                let response = serde_json::json!({"status":"error","message":"Success Login"});
                Ok(HttpResponse::Unauthorized().status(StatusCode::UNAUTHORIZED).json(response))
            }

        }
        Err(_) =>{
            Ok(HttpResponse::NotFound().status(StatusCode::NOT_FOUND).json(serde_json::json!({"status":"error","message":"No found record"})))
        }
    }
}
