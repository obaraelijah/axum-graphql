use crate::model::ServiceSchema;
use serde::Serialize;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::StatusCode, 
    response::IntoResponse, 
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

pub(crate) async fn graphql_handler(
    req: GraphQLRequest,
    Extension(schema): Extension<ServiceSchema>,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}