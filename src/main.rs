use std::env;

use actix_web::{web, App, HttpResponse, HttpServer};
use rust_precise::middleware::{JwtMiddleware, LoggingMiddleware};
use rust_precise::{settings::db_pool, module::auth::login};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db_pool().await;
    HttpServer::new(move||{
        let jwt_secret = env::var("JWT_SECRET").expect("No secret key in environment");
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(LoggingMiddleware)
            .wrap(JwtMiddleware::new(jwt_secret))
            .service(web::resource("/protected").route(web::get().to(protected_route)))
            .service(web::resource("/login").route(web::post().to(login)))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn protected_route() -> HttpResponse {
    HttpResponse::Ok().body("Protected Route")
}
