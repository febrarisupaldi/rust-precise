use std::env;

use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use docs::openapi::docs::ApiDoc;
use routes::routes::routes::all_routes;
use crate::middlewares::{logging_middleware, JwtMiddleware};
use crate::settings::db_pool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod modules;
pub mod routes;
pub mod schemas;
pub mod settings;
pub mod helpers;
pub mod docs;
pub mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db_pool().await;

    HttpServer::new(move||{
        let jwt_secret = env::var("JWT_SECRET").expect("No secret key in environment");
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(from_fn(logging_middleware::logging::log_middleware))
            .service(
                web::scope("/docs").service(
                    SwaggerUi::new("/api/{_:.*}").url("/openapi.json", ApiDoc::openapi())
                )
            )
            .route("/openapi.json", web::get().to(openapi_json))
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

async fn openapi_json() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(ApiDoc::openapi().to_pretty_json().unwrap())
}
