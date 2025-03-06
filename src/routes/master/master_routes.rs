pub mod master_routes{
    use actix_web::web;

    use crate::routes::master::{city_routes::city_routes, country_routes::country_routes, state_routes::state_routes};

    pub fn masters_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/master")
            .service(
                web::scope("")
                .configure(city_routes::cities_routes)
                .configure(country_routes::countries_routes)
                .configure(state_routes::states_routes)
            )
        );
    }
}