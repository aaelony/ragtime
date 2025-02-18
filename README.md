# ragtime

## Why this Project

I wanted to start putting together a portfolio of public showcase/demo projects in various domains and I was inspired by the Python project [deploy-rag-to-aws](https://github.com/pixegami/deploy-rag-to-aws) by [pixegami](https://github.com/pixegami). His project is intuitive, well-designed, and puts together an advanced use of AWS Lambda functions to call Amazon Bedrock LLMs to answer questions tailored to information only present in local PDF files.  

This project aims to re-implement the RAG architecture project but using the Rust language.

## Why Rust?

When using AWS and other cloud services, eventually recurring costs add up.  Because Rust compiles down to memory-efficient binaries and is known for it's speed compared to Python, faster, less memory-intensive Lambdas translate into greater performance and cost savings.

Even the author of the Oreilly book [Python for Devops](https://pythondevops.com/) recently wrote [Why I Like Rust Better Than Python](https://podcast.paiml.com/episodes/why-i-like-rust-better-than-python) which advocates for why Rust is an attractive choice over Python.

The polemnic why Rust is more efficient than Python at a high level, amounts to:

1. **Compiled vs. Interpreted**: Rust is a compiled language, producing optimized machine code, whereas Python is interpreted, which adds overhead during runtime.
2. **Memory Management**: Rust uses manual memory management with ownership, borrowing, and lifetimes, which avoids garbage collection and reduces runtime memory overhead.
3. **Concurrency**: Rust's ownership system ensures safe and efficient concurrency without race conditions, while Python's Global Interpreter Lock (GIL) limits multithreading performance.
4. **Low-Level Access**: Rust provides low-level control over hardware and memory, allowing better optimization for performance-critical tasks.
5. **Zero-Cost Abstractions**: Rust’s abstractions do not introduce performance penalties, unlike Python, which can incur overhead with higher-level constructs.

## Components

### AWS

 - [Amazon Lambda](https://aws.amazon.com/lambda/)
 - [Amazon Bedrock](https://aws.amazon.com/bedrock/)
 - 

### Rust Ecosystem

 - [crates.io](https://crates.io)
 - [cargo](https://doc.rust-lang.org/cargo/)
 - [cargo lambda](https://www.cargo-lambda.info/)

## Installation and Running the programs

### `vectordb_stuff`


### `lambda_stuff`





