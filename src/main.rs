use crate::model::QueryRoot;
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{
   extract::Extension, 
   routing::{get, post}, 
   Router, 
   Server, 
};

mod model;
mod routes;

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground))
        .route("/health", get(health))
        .layer(Extension(schema)); 
      
    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}