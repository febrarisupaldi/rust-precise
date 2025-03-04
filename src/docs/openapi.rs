pub mod docs{
    use utoipa::openapi::security::HttpAuthScheme;
    use utoipa::openapi::security::SecurityScheme;
    use utoipa::Modify;
    use utoipa::OpenApi;

    #[allow(unused_imports)]
    use crate::schemas::master::country_schema::{CountrySchema, InsertCountrySchema, UpdateCountrySchema, CountryCodeQuery, CountryNameQuery, 
        get_all_countries, create_country, update_country, get_country, exists_country_code, exists_country_name,
        __path_get_all_countries, __path_create_country, __path_update_country, __path_get_country, __path_exists_country_code, __path_exists_country_name};

    /// OpenAPI Documentation for Precise
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        security(
            ("bearerAuth"= [])
        ),
        paths(get_all_countries, create_country, update_country, get_country, exists_country_code, exists_country_name),
        components(schemas(CountrySchema, InsertCountrySchema, UpdateCountrySchema, CountryCodeQuery, CountryNameQuery)),
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

    pub struct ApiDoc;

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