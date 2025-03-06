use std::env;

use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use docs::docs_routes::docs_routes::{docs_routes, openapi_routes};
use routes::routes::routes::all_routes;
use crate::middlewares::{logging_middleware::logging::log_middleware, JwtMiddleware};
use crate::settings::db_pool;

pub mod modules;
pub mod routes;
pub mod schemas;
pub mod settings;
pub mod utils;
pub mod docs;
pub mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db_pool().await;
    HttpServer::new(move||{
        let jwt_secret = env::var("JWT_SECRET").expect("No secret key in environment");
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(from_fn(log_middleware))
            .service(
                web::scope("/docs").configure(docs_routes)
            )
            .service(
                web::scope("/openapi").configure(openapi_routes)
            )
            .service(
                web::scope("/precise/api")
                .wrap(JwtMiddleware::new(jwt_secret))
                .configure(all_routes)
            )
            .default_service(web::to(not_found))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn not_found() -> impl Responder{
    HttpResponse::NotFound().json(serde_json::json!({"status":"error","message":"Url Not found"}))
}


