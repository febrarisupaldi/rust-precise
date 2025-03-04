use std::{future::Future, pin::Pin};

use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, error::ErrorUnauthorized, Error, HttpMessage};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims{
    pub sub: String,
    pub exp: usize,
}

pub struct JwtMiddleware {
    jwt_secret: String,
}

impl JwtMiddleware{
    pub fn new(jwt_secret: String) -> Self{
        Self { jwt_secret }
    }
}



impl <S, B> Transform <S, ServiceRequest> for JwtMiddleware
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService{
            service,
            jwt_secret: self.jwt_secret.clone()
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    jwt_secret: String,
}

impl <S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    //forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.path() == "/precise/api/auth/login" {
            let fut = self.service.call(req);
            return Box::pin(async move{ fut.await});
        }

        let auth_header = req.headers()
            .get("Authorization")
            .and_then(|h|h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        if let Some(token) = auth_header{
            let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_bytes());

            match decode::<Claims>(token, &decoding_key, &Validation::default()){
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims);
                    let fut = self.service.call(req);
                    return Box::pin(async move{ fut.await});
                }
                Err(_) => {
                    return Box::pin(async move{
                        Err(ErrorUnauthorized(serde_json::json!({"status":"error","message": "Invalid token"}).to_string()))
                        //Err(ErrorUnauthorized(serde_json::json!({"status":"error","message": "Invalid token"}).to_string()))
                    });
                }
            }
        }

        Box::pin(async move{
            Err(ErrorUnauthorized(serde_json::json!({"status":"error","message": "You don't have access to endpoint"}).to_string()))
        })
    }
}

// pub fn config_jwt_middleware(cfg: &mut web::ServiceConfig){
//     let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

//     cfg.service(
//         web::scope("/api")
//         .wrap(JwtMiddleware::new(jwt_secret))
//         .service(web::resource("/protected").route(web::get().to(protected_route)))
//     );
// }

// async fn protected_route(claims: web::ReqData<Claims>) -> impl Responder {
//     HttpResponse::Ok().json(json!({"message": "You are authorized", "user_id": claims.sub}))
// }