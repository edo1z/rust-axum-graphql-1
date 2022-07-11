mod handler;

use async_graphql::*;
use axum::{extract::Extension, routing::get, Router};
use handler::{graphql_handler, graphql_playground};
use model::{dummy_data::Storage, MutationRoot, QueryRoot};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let storage = Arc::new(Mutex::new(Storage::new()));
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(storage)
        .finish();
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    axum::Server::bind(&"0.0.0.0:8090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
