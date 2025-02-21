
export S3_BUCKET_NAME=ragtime-bucket


COMMON_TESTS_CMD := cargo test -p common
COMMON_TESTS_CMD_SHOW_OUTPUT := cargo test -p common -- --nocapture
VECTORDB_BUILD_CMD := cargo build --release -p vectordb_stuff && ls -hl target/release/vectordb_stuff
CLEAR_DATABASE_CMD := target/release/vectordb_stuff --clear-database
LOAD_DOCUMENTS_CMD := target/release/vectordb_stuff --load-documents

LAMBDA_BUILD_CMD := cargo lambda build --release -p lambda_stuff && ls -hl target/lambda/lambda_stuff/bootstrap
DEPLOY_LAMBDA_CMD := cargo lambda deploy --enable-function-url  --env-var S3_BUCKET_NAME=ragtime-bucket  ## --iam-role IAMROLE

test_common:
	@echo "$(COMMON_TESTS_CMD)"
	@$(COMMON_TESTS_CMD)

test_common_show_output:
	@echo "$(COMMON_TESTS_CMD_SHOW_OUTPUT)"
	@$(COMMON_TESTS_CMD_SHOW_OUTPUT)


build_vectordb:
	@echo "$(VECTORDB_BUILD_CMD)"
	@$(VECTORDB_BUILD_CMD)

clear_database: build_vectordb
	@echo "$(CLEAR_DATABASE_CMD)"
	@$(CLEAR_DATABASE_CMD)

load_documents: clear_database
	@echo "$(LOAD_DOCUMENTS_CMD)"
	@$(LOAD_DOCUMENTS_CMD)




build_lambda:
	@echo "$(LAMBDA_BUILD_CMD)"
	@$(LAMBDA_BUILD_CMD)

deploy_lambda:
	@echo "$(DEPLOY_LAMBDA_CMD)"
	@$(DEPLOY_LAMBDA_CMD)
