pub mod master{
    use actix_web::web;

    use crate::routes::master::{city_routes::cities, country_routes::countries};

    pub fn master_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/master")
            .service(
                web::scope("")
                .configure(cities::cities_routes)
                .configure(countries::countries_routes)
            )
        );
    }
}