use aws_sdk_dynamodb::Client;
use lambda_http::{run, Body, Error, Request, Response, http::StatusCode};

use async_graphql::{
    Request as GraphQlRequest, Schema, EmptySubscription,
};

use lambda_runtime::service_fn;
use schema::{Query, Mutation};

mod schema;
mod db;

async fn function_handler(event: Request) -> Result<Response<Body>, String> {

    let config = aws_config::load_from_env().await;
    let dynamodb_client = Client::new(&config);

    let schema = Schema::build(Query, Mutation, EmptySubscription).data(dynamodb_client).finish();

    let query: Result<GraphQlRequest, String> = match event.into_body() {
        Body::Empty => Err("oops".to_owned()),
        Body::Text(text) => {
            serde_json::from_str::<GraphQlRequest>(&text).map_err(|_| "oops".to_owned())
        }
        Body::Binary(binary) => {
            serde_json::from_slice::<GraphQlRequest>(&binary).map_err(|_| "oops".to_owned())
        }
    };

    let query = match query {
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Text("Bad Request".to_owned()))
                .map_err(|_| "oops".to_owned())
        }
        Ok(query) => query,
    };

    let response_body = serde_json::to_string(&schema.execute(query).await)
        .map_err(|_| "oops".to_owned())?;

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::Text(response_body))
        .map_err(|_|"oops".to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting server...");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    run(service_fn(function_handler)).await?;

    Ok(())
}

