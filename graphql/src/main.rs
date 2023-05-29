use async_graphql::Request as GraphQlRequest;
use lambda_http::{http::StatusCode, run, Body, Error, Request, Response};
use lambda_runtime::service_fn;

mod db;
mod schema;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let schema = schema::build_schema().await;

    let query: GraphQlRequest = match event.into_body() {
        Body::Empty => {
            // Return a 400 bad request response
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Text("Bad Request".to_owned()))?);
        }
        Body::Text(text) => serde_json::from_str::<GraphQlRequest>(&text)?,
        Body::Binary(binary) => serde_json::from_slice::<GraphQlRequest>(&binary)?,
    };

    let body = serde_json::to_string(&schema.execute(query).await)?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::Text(body))?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await?;
    Ok(())
}
