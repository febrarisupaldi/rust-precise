pub mod user{

    use actix_web::web;
    use crate::modules::auth::login;
    
    pub fn user_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("")
            
            .service(
                web::resource("/login")
                    .route(web::post().to(login)))
        );
    }
}