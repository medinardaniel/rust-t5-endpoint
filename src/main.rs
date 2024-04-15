use lambda_http::{service_fn, run, Request, IntoResponse, Body, Error, Response};
use lambda_http::http::StatusCode;
use reqwest::Client;
use serde_json::{json, Value, from_str};
use tokio::time::{sleep, Duration};
use std::str;
use std::env;

async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let client = Client::new();
    let api_url = env::var("API_URL")?;
    let token = env::var("TOKEN")?;

    // Convert body from &[u8] to &str
    let body_bytes = event.body().as_ref();  // Get a reference to the &[u8]
    let body_str = str::from_utf8(body_bytes).unwrap(); // Convert &[u8] to &str
    let body: Value = from_str(body_str).unwrap();
    let inputs = body["command"].as_str().unwrap();

    let payload = json!({
        "inputs": inputs
    });

    let mut attempts = 0;
    let max_attempts = 3;
    let mut delay = Duration::from_secs(20); // Initial delay of 20 seconds as suggested by the API

    loop {
        let response = client.post(api_url.clone())
            .bearer_auth(token.clone())
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let body: Value = response.json().await?;
            if body["error"].is_string() && body["error"].as_str().unwrap().contains("loading") {
                if attempts < max_attempts {
                    attempts += 1;
                    sleep(delay).await;
                    delay *= 2; // Exponential backoff
                    continue;
                } else {
                    return Ok(Response::builder()
                        .status(StatusCode::SERVICE_UNAVAILABLE)
                        .body(Body::from("Model loading timeout"))
                        .expect("Failed to render response"));
                }
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(body.to_string()))
                    .expect("Failed to render response"));
            }
        } else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to reach model API"))
                .expect("Failed to render response"));
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
