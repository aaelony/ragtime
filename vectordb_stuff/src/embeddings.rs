use anyhow::Result;
use aws_sdk_bedrockruntime::primitives::Blob;
use aws_sdk_bedrockruntime::{
    // operation::converse::{ConverseError, ConverseOutput},
    // types::{ContentBlock, ConversationRole, Message},
    Client,
};
use serde_json::{json, Value};

pub async fn create_embeddings(question: &str, model_name: &str) -> anyhow::Result<Value> {
    let config = aws_config::load_from_env().await;
    let bedrock_client = Client::new(&config);

    let input_json = json!({
        "inputText": question
    });

    let input_bytes = serde_json::to_vec(&input_json)?;

    let response = bedrock_client
        .invoke_model()
        .body(Blob::new(input_bytes))
        .model_id(model_name)
        .content_type("application/json")
        .accept("application/json")
        .send()
        .await?;

    let response_body = response.body.as_ref();
    let embeddings: Value = serde_json::from_slice(response_body)?;

    Ok(embeddings)
}
