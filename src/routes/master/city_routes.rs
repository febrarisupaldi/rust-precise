pub mod cities{
    use actix_web::web;

    use crate::modules::master::city::city;

    pub fn cities_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/cities")
            
            .service(
                web::resource("")
                    .route(web::get().to(city::get_all_cities))
                    .route(web::post().to(city::insert_city))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(city::get_city))
                    .route(web::put().to(city::update_city))
            )
            .service(
                web::resource("/exists/name")
                    .route(web::get().to(city::exists_cities_name))
            )
            .service(
                web::resource("/exists/code")
                    .route(web::get().to(city::exists_cities_code))
            )
        );
    }
}