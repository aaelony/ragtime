mod cli;
mod pdftools;
mod vectordb;

use anyhow::Result;
use vectordb::VectorDb;

fn main() -> Result<()> {
    let cli = cli::parse_args();

    let vdb_client = VectorDb::new("embeddings.db")?;

    if cli.clear_database {
        println!("Clearing database...");
        vdb_client.drop_embeddings_table()?;
    }

    if cli.load_documents {
        println!("Loading documents...");
    }
    // Mode 1:
    //   Step 1: Reset the vector database.  --clear_database

    // Mode 2: --load_documents
    //  Step 2: Put any PDFs you want the model to reference in the `pdfs` directory.
    //  Step 3: Parse the PDF documents from pdfs directory
    //  Step 4: Add PDF documents to the vector database
    //  Step 5: You are now ready to search for similar documents and use the lambda.
    Ok(())
}
