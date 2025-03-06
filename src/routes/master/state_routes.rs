pub mod state_routes{
    use actix_web::web;

    use crate::modules::master::state::state;

    pub fn states_routes(cfg: &mut web::ServiceConfig){
        cfg.service(
            web::scope("/states")
            .service(
                web::resource("")
                .route(web::get().to(state::get_all_states))
                .route(web::post().to(state::create_state))
            )
            .service(
                web::resource("/{state_id}")
                .route(web::get().to(state::get_state))
                .route(web::put().to(state::update_state))
            )
            .service(
                web::scope("/exists")
                .service(
                    web::resource("/code")
                        .route(web::get().to(state::exists_state_code))
                )
                .service(
                    web::resource("/name")
                        .route(web::get().to(state::exists_state_name))
                )
            )
        );
    }   
}