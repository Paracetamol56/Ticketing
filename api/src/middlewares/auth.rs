use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    body::EitherBody,
    Error,
};
use futures_util::future::LocalBoxFuture;
use futures_util::{TryFutureExt, FutureExt};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AdminAuth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AdminAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let admin_token = std::env::var("ADMIN_TOKEN").unwrap_or_else(|_| "default_admin_token".to_string());
        ready(Ok(AdminAuthMiddleware { service, admin_token }))
    }
}

pub struct AdminAuthMiddleware<S> {
    service: S,
    admin_token: String,
}

impl<S, B> Service<ServiceRequest> for AdminAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Get the Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());
        
        // If the auth_token is different from the admin token,
        if Some(self.admin_token.as_str()) != auth_header {
            // Return a 403 Forbidden response
            let res = actix_web::HttpResponse::Forbidden().finish();
            return Box::pin(async { Ok(req.into_response(res.map_into_right_body())) });
        }

        self.service
            .call(req)
            .map_ok(ServiceResponse::map_into_left_body)
            .boxed_local()
    }
}

