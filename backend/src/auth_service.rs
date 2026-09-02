// This file was made as a test and is not actually used in the final version of the project

use std::{collections::HashSet, fmt, future::Ready, task::{Context, Poll}};

use axum::{body::Body, extract::Request, http::{self, Response}};
use futures_util::future::Either;
use tower::{Layer, Service};

use crate::config::ApiTokens;

#[derive(Debug, Clone)]
pub struct ApiKeyLayer {
    keys: HashSet<String>
}

#[derive(Debug, Clone)]
pub struct AuthService<S> {
    inner:S,
    keys:HashSet<String>
}

impl<S> Layer<S> for ApiKeyLayer {
    type Service = AuthService<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthService::new(service, self.keys.to_owned())
    }

}

impl ApiKeyLayer {
    pub fn new(tokens: ApiTokens) -> Self {
        ApiKeyLayer {keys: HashSet::from([tokens.token1.clone(), tokens.token2].clone())}
    }
}

impl<S> AuthService<S> {
    pub fn new(inner: S, keys: HashSet<String>) -> Self {
        AuthService { inner, keys }
    }
}

impl<S> Service<Request<Body>> for AuthService<S>
where
    S: Service<Request<Body>, Response = Response<Body>>,
    Request<Body>: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Either<S::Future, Ready<Result<S::Response, S::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let authorized = request
            .headers()
            .get("token")
            .and_then(|v| v.to_str().ok())
            .map(|token| self.keys.contains(token))
            .unwrap_or(false);

        if authorized {
            Either::Left(self.inner.call(request))
        } else {
            println!("Invalid or missing token");
            let response = http::Response::builder()
                .status(403)
                .body(Body::from("Invalid Unauthorized"))
                .unwrap();
            Either::Right(std::future::ready(Ok(response)))
        }
    }
}
