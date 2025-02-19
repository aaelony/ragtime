
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
	time cargo run -- --clear-database

load_documents:
	time cargo run -- --load-documents
