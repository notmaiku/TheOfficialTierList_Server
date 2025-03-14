use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use lambda_runtime::{lambda, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    // Add event fields as needed
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    status_code: i32,
    body: String,
}

async fn index(req: HttpRequest) -> impl Responder {
    format!(
        "Hello, world! (Request ID: {})",
        req.headers().get("x-request-id")
    )
}

#[lambda_runtime::handler]
async fn handler(_event: Event, _context: Context) -> Result<Response, lambda_runtime::Error> {
    // Implement your Lambda handler logic here
    // For example, call the index function and return the response
    let response = index(HttpRequest::default()).await;
    Ok(Response {
        status_code: 200,
        body: response.to_string(),
    })
}
