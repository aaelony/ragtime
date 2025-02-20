# lambda_stuff

This is intended to be a demo showcase project to illustrate how the Rust language can be used to create AWS Lambda functions and communicate with Amazon Bedrock.

1. Make sure you have the [rust toolchain and cargo](https://www.rust-lang.org/tools/install).
2. You will also need [cargo-lambda](https://www.cargo-lambda.info/guide/installation.html)
3. Optional: install `make` e.g. [Macos homebrew make](https://formulae.brew.sh/formula/make) or [linux make](https://askubuntu.com/questions/161104/how-do-i-install-make), or alternatively look at the `Makefile` for commands.

Then:

```
cd lambda_stuff
make build
```

If you want to run locally:
```
make watch
```
The above will run cargo lambda watch and allow you to edit code and have the compiler respond and rebuild when you save your work.


```
make deploy_on_aws_lambda
```

The above will deploy to AWS


You can ask a question to the model by sending a GET request. e.g.
```
http://localhost:9000/lambda-url/lambda_stuff?question_text="What is the capital of Arizona?"
```



