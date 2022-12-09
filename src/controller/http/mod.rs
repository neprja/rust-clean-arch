pub mod get_profile;

use crate::controller::http::get_profile::get_profile;
use crate::usecase::UseCase;
use anyhow::{Context, Result};
use axum::middleware::Next;
use axum::{
    extract::MatchedPath,
    http::{Request, StatusCode},
    response::Response,
};
use axum::{middleware, routing::get, Router, Server};
use opentelemetry::global;
use opentelemetry_http::HeaderExtractor;
use std::future::Future;
use std::net::SocketAddr;
use tracing::{field, info_span, Instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;

fn router(usecase: UseCase) -> Router {
    let probe = Router::new()
        .route("/healthz", get(probe))
        .route("/ready", get(probe));

    let v1 = Router::with_state(usecase)
        .route("/v1/profile/:id", get(get_profile))
        .route_layer(middleware::from_fn(opentelemetry));

    Router::new().merge(probe).merge(v1)
}

pub async fn server(
    port: u16,
    graceful_shutdown: impl Future<Output = ()>,
    usecase: UseCase,
) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    Server::bind(&addr)
        .serve(router(usecase).into_make_service())
        .with_graceful_shutdown(graceful_shutdown)
        .await
        .context("serve")?;

    Ok(())
}

async fn probe() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn opentelemetry<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
        path.as_str()
    } else {
        req.uri().path()
    };

    let parent = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(req.headers()))
    });

    let span = info_span!(
        "",
        otel.kind = "server",
        otel.name = format!("HTTP {} {}", req.method(), path),
        otel.status_code = field::Empty,
        otel.status_description = field::Empty,
        http.method = req.method().as_str(),
        http.status_code = field::Empty,
        http.route = path
    );

    span.set_parent(parent);

    let resp = next.run(req).instrument(span).await;

    Ok(resp)
}
