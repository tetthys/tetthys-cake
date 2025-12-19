use std::task::{Context as TaskContext, Poll};

use axum::http::Request;
use tower::{Layer, Service};

use crate::integration::cake::registry::{CakeConfig, CakeState};

#[derive(Clone)]
pub struct CakeLayer {
    state: CakeState,
}

impl CakeLayer {
    pub fn new(cfg: CakeConfig) -> Self {
        Self { state: CakeState::new(cfg) }
    }
}

impl<S> Layer<S> for CakeLayer {
    type Service = CakeSvc<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CakeSvc { inner, state: self.state.clone() }
    }
}

#[derive(Clone)]
pub struct CakeSvc<S> {
    inner: S,
    state: CakeState,
}

impl<S, B> Service<Request<B>> for CakeSvc<S>
where
    S: Service<Request<B>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        req.extensions_mut().insert(self.state.clone());
        self.inner.call(req)
    }
}
