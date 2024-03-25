use crate::model::ServiceSchema;
use serde::Serialize;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::StatusCode, 
    response::{Html, IntoResponse}, 
    Json
};


#[derive(Serialize)]
struct Health { 
    healthy: bool
}

pub(crate) async fn health() -> impl IntoResponse { 
    let health = Health {
        healthy: true
    };

    (StatusCode::OK, Json(health))
}

pub(crate) async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source( 
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

pub(crate) async fn graphql_handler(
    req: Json<GraphQLRequest>,
    Extension(schema): Extension<ServiceSchema>, // (2)
) -> GraphQLResponse {
    schema.execute(req.0.into_inner()).await.into() // (3)
}
