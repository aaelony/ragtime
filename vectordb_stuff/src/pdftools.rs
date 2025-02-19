// use anyhow::{Context, Result};

use pdf_extract;
use regex::Regex;
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

pub fn get_pdf_filenames(directory: String) -> Vec<String> {
    let dir_path = PathBuf::from(directory.clone());
    let mut pdf_files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension == "pdf" {
                    if let Some(filename) = entry.file_name().to_str() {
                        pdf_files.push(
                            Path::new(&directory)
                                .join(filename)
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                }
            }
        }
    }

    pdf_files
}

#[derive(Debug, Clone)]
pub struct ParsedPdf {
    pub filename: String,
    pub contents: String,
    pub chunks: Vec<String>,
}
// Text Processing
pub fn split_text_into_sentences(text: &str) -> Vec<String> {
    let text = text
        .replace('\n', " ")
        .replace('\r', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let re = Regex::new(r"[.!?][\s]+").unwrap();

    let sentences: Vec<String> = re
        .split(&text)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    sentences
}

pub struct TextSplitter {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub separators: Vec<&'static str>,
}

impl TextSplitter {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        TextSplitter {
            chunk_size,
            chunk_overlap,
            separators: vec!["\n\n", "\n", ". ", " ", ""],
        }
    }

    pub fn split_text(&self, text: &str) -> Vec<String> {
        let mut chunks = Vec::new();

        for separator in &self.separators {
            let splits: Vec<&str> = text.split(separator).collect();
            let mut current_chunk = String::new();

            for split in splits {
                if current_chunk.len() + split.len() > self.chunk_size {
                    if !current_chunk.is_empty() {
                        chunks.push(current_chunk.trim().to_string());
                    }

                    // Start new chunk with overlap - safer version
                    if self.chunk_overlap > 0 && !current_chunk.is_empty() {
                        let overlap_start = current_chunk.len().saturating_sub(self.chunk_overlap);
                        current_chunk =
                            current_chunk.get(overlap_start..).unwrap_or("").to_string();
                    } else {
                        current_chunk = String::new();
                    }
                }

                current_chunk.push_str(split);
                current_chunk.push_str(separator);
            }

            if !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
            }

            if !chunks.is_empty() {
                break;
            }
        }

        chunks
    }
}

pub fn extract_text_from_pdf(file_path: &str) -> io::Result<ParsedPdf> {
    let bytes = std::fs::read(file_path)?;
    let out = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let sentences = split_text_into_sentences(&out);
    //println!("This is the parsed text from {}: {}", file_path, out);
    for (i, s) in sentences.iter().enumerate() {
        println!("Sentence {i}: {s}")
    }

    //
    let splitter = TextSplitter::new(600, 120); //chunk size= 600, overlap= 120
    let chunks = splitter.split_text(&out);

    let obj = ParsedPdf {
        filename: file_path.to_string(),
        contents: out,
        chunks: chunks,
    };
    Ok(obj)
}
