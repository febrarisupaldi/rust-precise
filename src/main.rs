use actix_web::{web, App, HttpResponse, HttpServer};
use rust_precise::{settings::db_pool, module::login};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db_pool().await;
    HttpServer::new(move||{
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(HttpResponse::Ok))
            .route("/test", web::get().to(HttpResponse::Ok))
            .route("/login", web::post().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
