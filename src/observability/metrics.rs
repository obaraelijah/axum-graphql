use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::time::Instant;

const REQUEST_DURATION_METRIC_NAME: &str = "http_requests_duration_seconds";

pub(crate) fn create_prometheus_recorder() -> PrometheusHandle {
    unimplemented!()
}

pub(crate) async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    
}