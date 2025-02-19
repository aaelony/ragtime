use anyhow::{Context, Error, Result};
// use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client as S3Client;
use bytes::Bytes;
use lambda_http::{Body, Request, Response};
use rusqlite::Connection; // Result
use serde_json::Value;
use std::env;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct VectorDb {
    conn: Connection,
    local_path: String,
    s3_bucket: String,
    s3_key: String,
}

/// new() will create a new VectorDb instance with a connection to the local SQLite database.
impl VectorDb {
    pub async fn new(prefer_local: bool) -> Result<Self, anyhow::Error> {
        let local_path = String::from("/tmp/embeddings.db");
        let s3_bucket = env::var("S3_BUCKET_NAME")
            .map_err(|_| Error::msg("S3_BUCKET_NAME environment variable not set"))?;
        let s3_key = String::from("embeddings/embeddings.db");

        let should_download = !prefer_local || !Path::new(&local_path).exists();

        if should_download {
            println!("Downloading embeddings database from S3...");
            Self::download_from_s3(&local_path, &s3_bucket, &s3_key)
                .await
                .context("Failed to download database from S3")?;
        } else {
            println!("Using existing local database");
        }

        println!("Connecting to vector database at: {}", local_path);
        let conn = Connection::open(local_path.clone())?;
        Ok(VectorDb {
            conn,
            local_path,
            s3_bucket,
            s3_key,
        })
    }

    pub async fn push_to_s3(&self) -> Result<()> {
        let config = aws_config::load_from_env().await;
        let s3_client = S3Client::new(&config);
        let mut file = File::open(&self.local_path)
            .await
            .context(format!("Failed to open file: {}", self.local_path))?;

        // Read the file contents into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .context("Failed to read file contents")?;

        println!(
            "Uploading database file {} to S3: {}/{}",
            self.local_path, self.s3_bucket, self.s3_key
        );
        // Upload to S3
        s3_client
            .put_object()
            .bucket(&self.s3_bucket)
            .key(&self.s3_key)
            .body(buffer.into())
            .send()
            .await
            .context("Failed to upload file to S3")?;

        Ok(())
    }

    pub async fn download_from_s3(local_path: &str, s3_bucket: &str, s3_key: &str) -> Result<()> {
        let config = aws_config::load_from_env().await;
        let s3_client = S3Client::new(&config);

        // Get object from S3
        let response = s3_client
            .get_object()
            .bucket(s3_bucket)
            .key(s3_key)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to get object from S3: bucket={}, key={}",
                    s3_bucket, s3_key
                )
            })?;

        // Read the response body
        let body = response.body;
        let data: Bytes = body
            .collect()
            .await
            .context("Failed to read S3 response body")?
            .into_bytes();

        if let Some(parent) = std::path::Path::new(local_path).parent() {
            tokio::fs::create_dir_all(parent).await.with_context(|| {
                format!("Failed to create parent directories for {}", local_path)
            })?;
        }

        // Write to local file
        let mut file = File::create(local_path)
            .await
            .with_context(|| format!("Failed to create local file: {}", local_path))?;

        file.write_all(&data)
            .await
            .with_context(|| format!("Failed to write data to file: {}", local_path))?;

        Ok(())
    }

    pub fn is_local(&self) -> bool {
        if self.local_path.is_empty() {
            return false;
        }
        let path = Path::new(&self.local_path);
        path.exists() && path.is_file()
    }

    pub fn create_embeddings_table(&self) -> Result<()> {
        println!("Creating embeddings table if it doesn't exist...");
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY,
                text TEXT NOT NULL,
                embedding BLOB NOT NULL,
                metadata TEXT
            )",
                [],
            )
            .map_err(|e| {
                eprintln!("❌ Failed to create embeddings table: {}", e);
                anyhow::anyhow!("Database error: {}", e)
            })?;

        println!("✅ Successfully created embeddings table");
        Ok(())
    }

    pub fn drop_embeddings_table(&self) -> Result<()> {
        println!("Dropping embeddings table...");
        match self.conn.execute("DROP TABLE IF EXISTS embeddings", []) {
            Ok(_) => {
                println!("✅ Successfully dropped embeddings table");
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to drop embeddings table: {}", e);
                Err(e.into())
            }
        }
    }

    pub fn count_embeddings(&self) -> Result<i64> {
        let count = self
            .conn
            .query_row("SELECT COUNT(*) FROM embeddings", [], |row| row.get(0))?;
        Ok(count)
    }

    pub fn insert_embedding(
        &self,
        text: &str,
        embedding: &[f32],
        metadata: Option<&Value>,
    ) -> Result<i64> {
        let embedding_bytes: Vec<u8> = unsafe {
            std::slice::from_raw_parts(
                embedding.as_ptr() as *const u8,
                embedding.len() * std::mem::size_of::<f32>(),
            )
            .to_vec()
        };

        let metadata_str = metadata.map(|m| m.to_string());
        self.conn.execute(
            "INSERT INTO embeddings (text, embedding, metadata) VALUES (?1, ?2, ?3)",
            rusqlite::params![text, embedding_bytes, metadata_str],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn search_similar(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT text, embedding FROM embeddings")?;

        let mut results: Vec<(String, f32)> = Vec::new();

        let rows = stmt.query_map([], |row| {
            let text: String = row.get(0)?;
            let embedding_bytes: Vec<u8> = row.get(1)?;

            let embedding: Vec<f32> = unsafe {
                std::slice::from_raw_parts(
                    embedding_bytes.as_ptr() as *const f32,
                    embedding_bytes.len() / std::mem::size_of::<f32>(),
                )
                .to_vec()
            };

            Ok((text, embedding))
        })?;

        for row in rows {
            if let Ok((text, embedding)) = row {
                let similarity = cosine_similarity(query_embedding, &embedding);
                results.push((text, similarity));
            }
        }

        // Sort by similarity (highest first)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(limit);

        // Return only the strings
        Ok(results.into_iter().map(|(text, _)| text).collect())
    }
} // end of VectorDb impl

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_basic_functionality() {
        //let use_local_db = true;
        //let vdb_client = VectorDb::new(use_local_db).await?;
        //vdb_client.search_similar(question_embeddings, 5);
        // assert_eq!(2 + 2, 4);
    }
}
