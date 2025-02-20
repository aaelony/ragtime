# ragtime


## The Challenge 

The aim of this project is to implement use of an LLM from the Amazon Bedrock Marketplace such that the user can ask the model a question via an HTTP endpoint.
This is implemented as an AWS serverless Lambda written in Rust. 

An added challenge is to implement a [Retrieval Augmented Generation](https://en.wikipedia.org/wiki/Retrieval-augmented_generation) ([RAG](rag.md)) architecture, such that the LLM's responses are enhanced beyond the knowledge of what the base model knows alone.

The idea is to enable the application to enhance LLM responses by processing PDF files that you provide.
For example, you may have ancillary or proprietary information in PDF files that you want the LLM to be able to answer questions about.  The LLM can be enhanced by providing context to questions that the LLM alone could not possibly know.

On an as-needed basis, you can add PDF files to the [vectordf_stuff/pdfs](vectordb_stuff/pdfs) directory and the `vectordb_stuff` code will read the data, preprocess chunked text, and use Amazon Bedrock to create embeddings that are stored in a vector database. When a user asks a question, the question is processed and embeddings are created and compared to the embeddings in the database. The most similar embeddings are retrieved and added to the prompt for context that is sent to the LLM.

Notes:
 - You must enable authorization for the LLM you choose ahead of running the code.
 - Your default AWS configuration will be used for authentication.
 - You will need to enable permissions for S3, Lambda, and Bedrock.

## Why this Project?

I wanted to start putting together a portfolio of ambitious public showcase/demo projects in various domains and I was inspired by the Python project [deploy-rag-to-aws](https://github.com/pixegami/deploy-rag-to-aws) by [pixegami](https://github.com/pixegami). His python project is intuitive, well-designed, and puts together an advanced use of AWS Lambda functions to call Amazon Bedrock LLMs to answer questions tailored to information only present in local PDF files using Python.

This project aims to implement this idea, but in Rust.


## Why Rust?

Cloud services, while useful, eventually engender recurring costs that add up. Python ecosystem libraries are excellent in functionality but in practice often large in size.  Serverless architectures do have memory and size constraints as well as performance considerations.  Because cloud providers often charge by a formula along the lines of (`memory sizing` x `time spent`), a smaller memory footprint and speedy execution will translate to decreased costs over time.

Notable that an author of the Oreilly book [Python for Devops](https://pythondevops.com/) recently wrote [Why I Like Rust Better Than Python](https://podcast.paiml.com/episodes/why-i-like-rust-better-than-python) which advocates for why Rust is an attractive choice over Python.



## Project Status:

 - [X] DONE: AWS Lambda that communicates with LLM via AWS Bedrock
 - [X] DONE: Ability to read, parse, chunk, compute and store embeddings into a local database.
 - [ ] TODO: (50% DONE) RAG Retrieval of document embeddings to compare for similarity with a user question and provide context for the prompt.
 - [ ] TODO: More tests, polish, and possibly a UI.






## Components

### AWS

 - [Amazon Lambda](https://aws.amazon.com/lambda/)
 - [Amazon Bedrock](https://aws.amazon.com/bedrock/)
 - [Amazon S3](https://aws.amazon.com/s3/)

### Rust Ecosystem

 - [crates.io](https://crates.io)
 - [cargo](https://doc.rust-lang.org/cargo/)
 - [cargo lambda](https://www.cargo-lambda.info/)

## Installation and Running the programs

For run instructions, please see the README.md files linked below.

 1. [vectordb_stuff](vectordb_stuff/README.md) - containing PDF document processing and the embeddings database.
 2. [lambda_stuff](lambda_stuff/README.md) - AWS lambda handler function.
 3. [common](common/README.md) - code that is shared by the 2 components above.


