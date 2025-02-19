use crate::bedrock::ask_bedrock;
use anyhow::Result;
use aws_config::from_env;
use aws_sdk_s3::Client;
// use chrono::{DateTime, Utc};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde_json::{json, Value};
use std::env;
use std::path::Path;
use tokio::fs;

fn clean_text(text: &str) -> String {
    let new_string: String = text.replace("\n", "<p/>").to_string();
    new_string
}

async fn upload_embeddings_to_s3(
    db_path: &Path,
    s3_client: &Client,
    s3_bucket: &str,
    s3_key: &str,
) -> Result<(), Error> {
    // let s3_bucket = "your-bucket-name";
    // let s3_key = "embeddings/embeddings.db";

    // Check if file exists
    if !db_path.exists() {
        return Err(Error::from("Embeddings database file not found"));
    }

    // Read the file
    let data = fs::read(db_path)
        .await
        .map_err(|e| Error::from(format!("Failed to read embeddings file: {}", e)))?;

    // Upload to S3
    s3_client
        .put_object()
        .bucket(s3_bucket)
        .key(s3_key)
        .body(data.into())
        .send()
        .await
        .map_err(|e| Error::from(format!("Failed to upload to S3: {}", e)))?;

    tracing::info!("Successfully uploaded embeddings database to S3");
    Ok(())
}

/// This is the main body for the AWS Lambda function.
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // let aws_region = "us-west-2";
    let embeddings_model_name = "amazon.titan-embed-text-v2:0";
    let model_name = "anthropic.claude-3-5-haiku-20241022-v1:0";
    let query = event.query_string_parameters();

    let vectordb_path = Path::new("../vectordb_stuff/embeddings.db");
    let config = aws_config::load_from_env().await;
    let s3_client = Client::new(&config);

    let s3_bucket = env::var("S3_BUCKET_NAME")
        .map_err(|_| Error::from("S3_BUCKET_NAME environment variable not set"))?;
    let s3_key = "embeddings/embeddings.db";

    println!("About to upload embeddings to S3 {s3_bucket}/{s3_key}");
    upload_embeddings_to_s3(vectordb_path, &s3_client, &s3_bucket, s3_key).await?;

    let (question_text, answer) = match query.first("question_text") {
        Some(question_text) => {
            let response = ask_bedrock(question_text, model_name, embeddings_model_name).await?;
            (question_text, response)
        },
        None => (
            "Hello, please refer to the provided instructions",
            String::from("{\"answer\":\"Hello, please refer to the provided instructions\",\"metadata\":{\"input_tokens\":0,\"output_tokens\":0,\"total_tokens\":0,\"model\":\"\",\"timestamp\":\"\"},\"question\":\"\"}")
        ),
    };

    // let question_text = match query.first("question_text") {
    //     Some(question_text) => question_text,
    //     None => "Hello, please refer to the provided instructions",
    // };
    // let answer = ask_bedrock(question_text, model_name).await?;
    let message = format!("Response: {answer}");

    //experimental
    let parsed: serde_json::Value = serde_json::from_str(&answer).unwrap();
    let html_message = format!(
        r#"
        <html>
        <style>
                .response-container {{
                    font-family: Arial, sans-serif;
                    margin: 20px;
                    padding: 15px;
                    border: 1px solid #ddd;
                    border-radius: 5px;
                }}
                .question, .answer, .metadata {{
                    margin: 10px 0;
                }}
                .metadata ul {{
                    list-style-type: none;
                    padding-left: 20px;
                }}
                .metadata li {{
                    margin: 5px 0;
                }}
        </style>
        <body>
        <div class="response-container">
            <div class="question">
                <strong>Question:</strong> {}
            </div>
            <div class="answer">
                <strong>Answer:</strong> {}
            </div>
            <div class="metadata">
                <strong>Metadata:</strong>
                <ul>
                    <li>Input Tokens: {}</li>
                    <li>Output Tokens: {}</li>
                    <li>Total Tokens: {}</li>
                    <li>Model: {}</li>
                    <li>Timestamp: {}</li>
                </ul>
            </div>
        </div>
        </body></html>
        "#,
        clean_text(parsed["question"].as_str().unwrap_or("")),
        clean_text(parsed["answer"].as_str().unwrap_or("")),
        parsed["metadata"]["input_tokens"],
        parsed["metadata"]["output_tokens"],
        parsed["metadata"]["total_tokens"],
        clean_text(parsed["metadata"]["model"].as_str().unwrap_or("")),
        parsed["metadata"]["timestamp"]
    );

    //end experimental

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(html_message.into())
        //.body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{Request, RequestExt};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello world, this is an AWS Lambda HTTP request"
        );
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "lambda_stuff".into());

        let request = Request::default().with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello lambda_stuff, this is an AWS Lambda HTTP request"
        );
    }
}
