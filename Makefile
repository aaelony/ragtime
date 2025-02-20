
export S3_BUCKET_NAME=ragtime-bucket


VECTORDB_BUILD_CMD := cargo build --release -p vectordb_stuff && ls -hl target/release/vectordb_stuff
LAMBDA_BUILD_CMD := cargo lambda build --release -p lambda_stuff && ls -hl target/lambda/lambda_stuff/bootstrap
DEPLOY_LAMBDA_CMD := cargo lambda deploy --enable-function-url  --env-var S3_BUCKET_NAME=ragtime-bucket  ## --iam-role IAMROLE


build_vectordb:
	@echo "$(VECTORDB_BUILD_CMD)"
	@$(VECTORDB_BUILD_CMD)


build_lambda:
	@echo "$(LAMBDA_BUILD_CMD)"
	@$(LAMBDA_BUILD_CMD)

deploy_lambda:
	@echo "$(DEPLOY_LAMBDA_CMD)"
	@$(DEPLOY_LAMBDA_CMD)
