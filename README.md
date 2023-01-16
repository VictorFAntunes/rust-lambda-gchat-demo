# Rust Lambda Function for Google Chat Integration

This project is a demonstration of using the Rust programming language and the `lambda_runtime` crate to handle an AWS Lambda event, and the `reqwest` crate to send a message to a Google Chat webhook, and how to test it locally with hot-reload.

## Prerequisites
- Install [Rust](https://www.rust-lang.org/learn/get-started)
- Install [cargo-lambda](https://github.com/awslabs/aws-lambda-rust-runtime#installing-the-cargo-lambda-tool)

## Running Locally

1. Replace the mock `WEBHOOK_URL` environment variable defined under `[package.metadata.lambda.env]` in the `cargo.toml` file with your actual webhook URL. To create a Google Chat webhook, you can follow the instructions [here](https://developers.google.com/hangouts/chat/how-tos/webhooks).
2. Run `cargo lambda watch` from the root directory to start the local development server
3. Use the `cargo lambda invoke` command to send a test event to the server. For example:

```
cargo lambda invoke --data-ascii '{
"workflow": "Automated TCC",
"exc_id": "12345",
"categories": ["CD_OPS", "AM_DEV"],
"message": "Workflow failed, but dont worry",
"continue_url": "https://example.com/continue",
"abort_url": "https://example.com/abort"
}'
```

This should send a message to your Google Chat room with a card containing the information extracted from the event.

**Note**: This project is a demonstration of the Rust `lambda_runtime` and `cargo-lambda` use and is not meant to be immediately deployed to production, however if you'd like to test it in an AWS environment the `cargo lambda deploy` command is an easy way to do it.

## Additional Resources:
- [AWS Lambda Rust Runtime introduction videos](https://www.cargo-lambda.info/guide/screencasts.html)
- [cargo Lambda reference](https://www.cargo-lambda.info/guide/getting-started.html)
- [lambda_runtime](https://docs.rs/lambda_runtime/)
- [tracing](https://docs.rs/tracing/)
- [reqwest](https://docs.rs/reqwest/)

## Awesome extra features of Rust in AWS:

- [Official Rust AWS SDK](https://github.com/awslabs/aws-sdk-rust) with Clients to most AWS services (can be used from the lamda runtime, just like our http Client in this example)

- [Rust Structs for most AWS events](https://github.com/calavera/aws-lambda-events) that allow for easy serialization and deserialization of messages between AWS services.
