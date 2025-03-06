pub mod state_openapi{
    use utoipa::openapi::security::HttpAuthScheme;
    use utoipa::openapi::security::SecurityScheme;
    use utoipa::Modify;
    use utoipa::OpenApi;

    #[allow(unused_imports)]
    use crate::schemas::master::state_schema::{
        StatesSchema, InsertStatesSchema, UpdateStatesSchema,
        get_all_states, get_state, create_state, update_state,
        __path_get_all_states, __path_get_state, __path_create_state, __path_update_state
    };

    /// OpenAPI Documentation for Precise
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        security(
            ("bearerAuth"= [])
        ),
        paths(get_all_states, get_state, create_state, update_state),
        components(schemas(StatesSchema, InsertStatesSchema, UpdateStatesSchema)),
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

    pub struct StateApiDoc;

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