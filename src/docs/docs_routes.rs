pub mod docs_routes{
    use actix_web::{web, HttpResponse, Responder};
    use serde_json::Value;
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;
    use crate::docs::master::{country_openapi::country_openapi::CountryApiDoc, state_openapi::state_openapi::StateApiDoc, city_openapi::city_openapi::CityApiDoc};


    pub fn docs_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/master")
            .service(
                SwaggerUi::new("/api/country/{_:.*}").url(path_json_file("master","country"), CountryApiDoc::openapi())
            )
            .service(
                SwaggerUi::new("/api/state/{_:.*}").url(path_json_file("master","state"), StateApiDoc::openapi())
            )
            .service(
                SwaggerUi::new("/api/city/{_:.*}").url(path_json_file("master","city"), CityApiDoc::openapi())
            )
        );
    }

    pub fn openapi_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("")
            .route(&path_json_file("master","country"), web::get().to(openapi_json::<CountryApiDoc>))
            .route(&path_json_file("master","state"), web::get().to(openapi_json::<StateApiDoc>))
            .route(&path_json_file("master","city"), web::get().to(openapi_json::<CityApiDoc>))
        );
    }

    fn path_json_file(path: &str, file_name: &str) -> String {
        format!("/{}/openapi-{}.json", path, file_name)
    }

    async fn openapi_json<T:OpenApi>() -> impl Responder {
        let json_value: Value = serde_json::from_str(&T::openapi().to_pretty_json().unwrap()).unwrap();
        HttpResponse::Ok().json(json_value)
    }
}