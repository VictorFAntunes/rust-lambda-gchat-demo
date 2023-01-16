use google_chat_integration::{create_card_message, AMErrorEvent, Response};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use reqwest::Client;
use tracing::info;

/// pub struct AMErrorEvent {
///     workflow: String,
///     exc_id: String,
///     categories: Vec<String>,
///     message: String,
///     continue_url: Option<String>,
///     abort_url: Option<String>,
/// }

/// pub struct Response {
///     pub req_id: String,
///     pub message: String,
/// }

// Asynchronous function to handle the event and send the message
async fn function_handler(
    event: LambdaEvent<AMErrorEvent>,
    http_client: &Client,
) -> Result<Response, Error> {
    // Generate the message to be sent.
    let json_payload = create_card_message(&event.payload);
    // Obtain the Webhook endpoint from env.
    let webhook_url = std::env::var("WEBHOOK_URL")
        .map_err(|_| Error::from("Missing WEBHOOK_URL environment variable"))?;
    // Create web client in main since it's more efficient and can be reused in multiple invocations
    // let http_client = Client::new();

    let gchat_response = http_client
        .post(webhook_url)
        .json(&json_payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if gchat_response.status().is_success() {
        info!("Message sent!");
        Ok(Response {
            req_id: event.context.request_id,
            message: format!("Message sent"),
        })
    } else {
        let status_code = gchat_response.status().as_u16();
        let message = format!("Failed to send message. status code:{}", status_code);
        info!(message);
        Err(Error::from(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create the Client in main so it can be reused while the lambda is up
    let http_client = Client::new();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(|event: LambdaEvent<AMErrorEvent>| {
        function_handler(event, &http_client)
    }))
    .await
}
