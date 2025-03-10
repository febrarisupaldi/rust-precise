pub mod routes{
    use actix_web::web;

    use crate::routes::{auth::auth_routes::auth, master::master_routes::master_routes};

    pub fn all_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("")
            .service(
                web::scope("")
                .configure(auth::auth_routes)
                .configure(master_routes::masters_routes)
            )
        );
    }
}