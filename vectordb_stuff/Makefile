
export S3_BUCKET_NAME=ragtime-bucket

help:
	cargo run -- --help

fmt:
	cargo fmt

test:
	cargo test

build: fmt
	cargo build

run: build
	cargo run

clear_database:
	cargo run -- --clear-database

load_documents:
	cargo run -- --load-documents
