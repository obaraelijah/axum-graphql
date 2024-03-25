use crate::model::QueryRoot;
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, middleware, routing::get, Router, Server};
use observability::tracing::create_tracer_from_env;
use std::future::ready;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

mod model;
mod observability;
mod routes;

#[tokio::main]
async fn main() {

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let prometheus_recorder = create_prometheus_recorder();

    let registry = Registry::default().with(tracing_subscriber::fmt::layer().pretty());

    match create_tracer_from_env() {
        Some(tracer) => registry
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()
            .expect("Failed to register tracer with registry"),
        None => registry
            .try_init()
            .expect("Failed to register tracer with registry"),
    }

    info!("Service starting");

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health))
        .route("/metrics", get(move || ready(prometheus_recorder.render())))
        .route_layer(middleware::from_fn(track_metrics))
        .layer(Extension(schema));

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}