pub mod countries{
    use actix_web::web;

    use crate::modules::master::country::country;

    pub fn countries_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/countries")
            
            .service(
                web::resource("")
                    .route(web::get().to(country::get_all_countries))
                    .route(web::post().to(country::create_country))
            )
            .service(
                web::resource("/{id}")
                    .route(web::put().to(country::update_country))
                    .route(web::get().to(country::get_country))
            )
            .service(
                web::scope("/exists")
                .service(
                    web::resource("/code")
                        .route(web::get().to(country::exists_country_code))
                )
                .service(
                    web::resource("/name")
                        .route(web::get().to(country::exists_country_name))
                )
            )
        );
    }
}