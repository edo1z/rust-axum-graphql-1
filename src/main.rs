mod handler;

use async_graphql::*;
use axum::{extract::Extension, routing::get, Router};
use handler::{graphql_handler, graphql_playground};
use model::{MutationRoot, QueryRoot};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    axum::Server::bind(&"0.0.0.0:8090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
