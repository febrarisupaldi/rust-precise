pub mod city_openapi{
    use utoipa::openapi::security::HttpAuthScheme;
    use utoipa::openapi::security::SecurityScheme;
    use utoipa::Modify;
    use utoipa::OpenApi;

    use crate::schemas::master::city_schema::*;

    #[allow(unused_imports)]
    use crate::schemas::master::city_schema::{__path_get_all_cities, __path_get_city, __path_create_city, __path_update_city, __path_exists_city_code, __path_exists_city_name};

    /// OpenAPI Documentation for Precise
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        security(
            ("bearerAuth"= [])
        ),
        paths(get_all_cities, get_city, create_city, update_city, exists_city_code, exists_city_name),
        components(schemas(CitiesSchema, InsertCitySchema, UpdateCitySchema, CityCodeQuery, CityNameQuery)),
        tags(
            (name = "Precise API", description="API for Precise")
        ),
        info(
            title = "Precise API Service",
            version = "1.0.0",
            license(
                name = "MIT"
            ),
            contact(
                name = "PT Presindo Central",
                url = "https://www.onyxhouseware.com",
                email = "smart.presindo@gmail.com"
            ),
            description = "Precise API for PT Presindo Central"
        ),
        
    )]
    pub struct CityApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon{
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut(){
                components.add_security_scheme("bearer_auth",
                    SecurityScheme::Http(utoipa::openapi::security::Http::new(HttpAuthScheme::Bearer))
                );
            }
        }
    }
}