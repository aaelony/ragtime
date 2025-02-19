use anyhow::Result;
use rusqlite::Connection; // Result
use serde_json::Value;

pub struct VectorDb {
    conn: Connection,
}

impl VectorDb {
    pub fn new(path: &str) -> Result<Self> {
        println!("Connecting to vector database at: {}", path);
        let conn = Connection::open(path)?;
        Ok(VectorDb { conn })
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

    pub fn search_similar(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<(String, f32)>> {
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

        Ok(results)
    }
} // end of VectorDb impl

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}
