// use anyhow::Result;
use aws_sdk_bedrockruntime::primitives::Blob;
use aws_sdk_bedrockruntime::{
    // operation::converse::{ConverseError, ConverseOutput},
    // types::{ContentBlock, ConversationRole, Message},
    Client,
};
// use lambda_http::{Body, Error, Request, Response};
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
    println!("embeddings: {:?}", embeddings);

    Ok(embeddings)
}

pub fn convert_embeddings_to_f64(embeddings: &Value) -> anyhow::Result<Vec<f64>> {
    let embeddings_array = embeddings["embedding"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Embeddings should contain an 'embedding' array"))?;

    let mut embeddings = Vec::with_capacity(embeddings_array.len());
    for value in embeddings_array {
        let f64_value = value
            .as_f64()
            .ok_or_else(|| anyhow::anyhow!("Each embedding value should be a number"))?;
        embeddings.push(f64_value as f64);
    }

    Ok(embeddings)
}

// Tests section
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vectordb::cosine_similarity;

    #[tokio::test]
    async fn test_create_embeddings_and_similarity() -> anyhow::Result<()> {
        let question = "My favorite color is green";
        let embeddings_model_name = "amazon.titan-embed-text-v2:0";
        let embeddings = create_embeddings(question, embeddings_model_name).await?;
        assert!(embeddings.is_object(), "Response should be a JSON object");

        let embeddings_array = embeddings["embedding"]
            .as_array()
            .expect("Embeddings should contain an 'embedding' array");

        assert!(
            !embeddings_array.is_empty(),
            "Embeddings array should not be empty"
        );
        assert_eq!(
            embeddings_array.len(),
            1024,
            "Expected embedding dimension is 1024"
        );

        // Verify all elements are numbers
        for value in embeddings_array {
            assert!(value.is_number(), "Each embedding value should be a number");
        }

        // Similarity test
        let similar_text = "I like the color green.";
        let dissimilar_text = "Rabbits are fast.";
        let similar_embeddings = create_embeddings(similar_text, embeddings_model_name).await?;
        let dissimilar_embeddings =
            create_embeddings(dissimilar_text, embeddings_model_name).await?;

        let similar_embeddings_f64 = convert_embeddings_to_f64(&similar_embeddings)?;
        let dissimilar_embeddings_f64 = convert_embeddings_to_f64(&dissimilar_embeddings)?;
        let question_embeddings_f64 = convert_embeddings_to_f64(&embeddings)?;

        let similarity_score = cosine_similarity(&question_embeddings_f64, &similar_embeddings_f64);
        let dissimilarity_score =
            cosine_similarity(&question_embeddings_f64, &dissimilar_embeddings_f64);
        let dissimilarity_score2 =
            cosine_similarity(&similar_embeddings_f64, &dissimilar_embeddings_f64);

        println!(
            "Similarity between {} and {}: {}",
            question, similar_text, similarity_score
        );
        println!(
            "Similarity between {} and {}: {}",
            question, dissimilar_text, dissimilarity_score
        );
        println!(
            "Similarity between {} and {}: {}",
            similar_text, dissimilar_text, dissimilarity_score2
        );

        assert!(similarity_score > dissimilarity_score, "The similarity score for '{}' and '{}' ({}) should be greater than for '{}' and '{}' ({})",
        question, similar_text, similarity_score,
        question, dissimilar_text, dissimilarity_score
        );

        Ok(())
    }
}
