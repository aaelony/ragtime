// use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::{
    operation::converse::{ConverseError, ConverseOutput},
    types::{ContentBlock, ConversationRole, Message},
    Client,
};
use aws_smithy_types::Blob;
use chrono;
use serde_json::{json, Value};

// based on examples found here: https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rustv1/examples/bedrock-runtime/src/bin/converse.rs

#[derive(Debug)]
struct BedrockConverseError(String);
impl std::fmt::Display for BedrockConverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't invoke bedrock model. Reason: {}", self.0)
    }
}
impl std::error::Error for BedrockConverseError {}
impl From<&str> for BedrockConverseError {
    fn from(value: &str) -> Self {
        BedrockConverseError(value.to_string())
    }
}
impl From<&ConverseError> for BedrockConverseError {
    fn from(value: &ConverseError) -> Self {
        BedrockConverseError::from(match value {
            ConverseError::ModelTimeoutException(_) => "Model took too long",
            ConverseError::ModelNotReadyException(_) => "Model is not ready",
            _ => "Unknown",
        })
    }
}

fn get_converse_output_text(output: ConverseOutput) -> Result<String, BedrockConverseError> {
    let message = output
        .output()
        .ok_or(BedrockConverseError("No output content".into()))?
        .as_message()
        .map_err(|_| BedrockConverseError("Output not a message".into()))?;

    let text = message
        .content()
        .first()
        .and_then(|content| content.as_text().ok())
        .ok_or(BedrockConverseError(
            "No text content found in message".into(),
        ))?;

    Ok(text.to_string())
}

pub struct QueryModel {
    pub question_text: Option<String>,
    pub answer_text: Option<String>,
    // utc_created_at: Datetime<Utc>,
}

pub async fn create_embeddings(
    question: &str,
    model_name: &str,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let input_json = json!({
        "inputText": question
    });

    let input_bytes = serde_json::to_vec(&input_json)?;

    let response = client
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

// Ask Bedrock a question for the LLM to answer
pub async fn ask_bedrock(
    question: &str,
    model_name: &str,
    embeddings_model_name: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let question_embeddings = create_embeddings(question, embeddings_model_name).await?;

    let prompt = format!("Human: {}\n\nAssistant: ", question);
    let prompt_clone = prompt.clone();

    let response_output = client
        .converse()
        .model_id(model_name)
        .messages(
            Message::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(prompt_clone))
                .build()
                .map_err(|_| "failed to build message")?,
        )
        .send()
        .await?;

    // Create JSON with answer and metadata
    // https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_Converse.html
    // https://docs.rs/aws-sdk-bedrockruntime/latest/aws_sdk_bedrockruntime/types/struct.ConverseMetrics.html
    let response_json = json!({
            "answer": match &response_output.output {
                Some(output) => get_converse_output_text(response_output.clone())?,
                None => return Err("No output content found".into()),
            },
            "metadata": {
                "model": model_name,
                "prompt": prompt.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "input_tokens": response_output.usage.as_ref().map(|u| u.input_tokens).unwrap_or(0),
                "output_tokens": response_output.usage.as_ref().map(|u| u.output_tokens).unwrap_or(0),
                "total_tokens": response_output.usage.as_ref()
                    .map(|u| u.input_tokens + u.output_tokens)
                    .unwrap_or(0),
            },
            "question": question.to_string(),
    });

    // Convert to string
    let json_string = serde_json::to_string(&response_json)?;

    Ok(json_string.to_string())
}
