use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[command(author,
     version = env!("CARGO_PKG_VERSION"),  // This pulls the version from Cargo.toml
     about = "Set up the vector database for the RAG project.",
     long_about = None)]
pub struct Cli {
    /// Clear the database before proceeding
    #[arg(long)]
    pub clear_database: bool,

    /// Load documents into the database from the local `pdfs` directory
    #[arg(long)]
    pub load_documents: bool,
}

pub fn parse_args() -> Cli {
    let cli = Cli::parse();
    if !cli.clear_database && !cli.load_documents {
        Cli::command().print_help().unwrap();
        std::process::exit(1);
    }
    cli
}
