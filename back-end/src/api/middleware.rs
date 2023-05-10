// use std::future::{ready, Ready};
// use crate::{api::response::MyResponse};
// use actix_web::{
//     dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
//     Error,
//     http::{header},
//     HttpResponseBuilder
// };
// use futures_util::future::LocalBoxFuture;
// use std::collections::HashMap;
// use serde::Serialize;

// // There are two steps in middleware processing.
// // 1. Middleware initialization, middleware factory gets called with
// //    next service in chain as parameter.
// // 2. Middleware's call method gets called with normal request.
// pub struct middleware;

// // Middleware factory is `Transform` trait
// // `S` - type of the next service
// // `B` - type of response's body
// impl<S, B> Transform<S, ServiceRequest> for middleware
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = SayHiMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(SayHiMiddleware { service }))
//     }
// }

// pub struct SayHiMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

//     forward_ready!(service);

//     fn call(&self, req: ServiceRequest) -> Self::Future {

//         let fut = self.service.call(req);

//         Box::pin(async move {
//             let res = fut.await?;

//             let new_request = res.request().clone();

//             let status_code = res.response_mut().status();

//             let new_response = HttpResponseBuilder::new(status_code)
//                     .insert_header((header::CONTENT_TYPE, "application/json")).json(MyResponse {
//                         result: status_code.to_string(),
//                         message: "abc".to_string(),
//                         data: None
//                     });

//             // Ok(ServiceResponse::new(
//             //     new_request,
//             //     new_response
//             // ))
//             Ok(res)
//         })
//     }
// }