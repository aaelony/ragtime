
export S3_BUCKET_NAME=ragtime-bucket

build:
	cargo lambda build --release
	ls -hl target/lambda/lambda_stuff/bootstrap

build_for_graviton:
	cargo lambda build --release --arm64
	ls -hl target/lambda/lambda_stuff/bootstrap

## Watch will start a local server: localhost:9000 to emulate what the lambda will do once deployed.
watch:
	cargo lambda watch

deploy_on_aws_lambda: build
	cargo lambda deploy --enable-function-url  --env-var S3_BUCKET_NAME=ragtime-bucket  ## --iam-role IAMROLE

test_curl_question:
	curl -v -G "http://localhost:9000" --data-urlencode "question_text=What is the capital of California?"
