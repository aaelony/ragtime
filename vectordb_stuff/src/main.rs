mod cli;
mod embeddings;
mod pdftools;
mod vectordb;
use anyhow::Result;
use embeddings::create_embeddings;
use pdftools::{extract_text_from_pdf, get_pdf_filenames};
use serde_json::Value;
use vectordb::VectorDb;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::parse_args();

    let vdb_client = VectorDb::new("embeddings.db")?;

    // Mode 1:
    //   Step 1: Reset the vector database.  --clear_database
    if cli.clear_database {
        println!("Clearing database...");
        vdb_client.drop_embeddings_table()?;
    }

    // Mode 2: --load_documents
    //  Step 1: Put any PDFs you want the model to reference in the `pdfs` directory.
    //  Step 2: Parse the PDF documents from pdfs directory
    //  Step 3: Add PDF documents to the vector database
    //  Step 4: Ready to search for similar documents and use the lambda.
    if cli.load_documents {
        println!("Loading documents...");
        let pdf_dir = "pdfs".to_string();
        let embeddings_model_name = "amazon.titan-embed-text-v2:0".to_string();
        let pdf_filenames = get_pdf_filenames(pdf_dir);
        let mut parsed_pdf_files = Vec::new();

        for (i, pdf_filepath) in pdf_filenames.iter().enumerate() {
            println!(
                "Processing {}/{}: {}",
                i + 1,
                pdf_filenames.len(),
                pdf_filepath
            );
            let parsed_pdf = extract_text_from_pdf(pdf_filepath.as_str())?;
            parsed_pdf_files.push(parsed_pdf.clone());
            vdb_client.create_embeddings_table()?;

            println!("Preparing to add documents to vector database...");
            for chunk in &parsed_pdf.chunks {
                let embeddings = create_embeddings(chunk, &embeddings_model_name).await?;

                let embedding_vec: Vec<f32> = embeddings
                    .get("embedding") // Get the "embedding" field from the response object
                    .and_then(|e| e.as_array())
                    .ok_or_else(|| anyhow::anyhow!("Embeddings field not found or not an array"))?
                    .iter()
                    .map(|v| v.as_f64().unwrap_or_default() as f32)
                    .collect();

                println!(
                    "Inserting embedding {:?} for {} into database...",
                    embedding_vec, chunk
                );
                vdb_client.insert_embedding(&chunk, &embedding_vec, None)?;
            } // end for loop that creates embeddings from text chunks and inserts into db
        } // end for loop pdf filenames
    }

    Ok(())
}
