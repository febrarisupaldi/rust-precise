pub mod auth{

    use actix_web::web;
    use crate::routes::auth::user_routes::user;
    
    pub fn auth_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/auth")
            
            .service(
                web::scope("")
                .configure(user::user_routes)
            )
        );
    }
}