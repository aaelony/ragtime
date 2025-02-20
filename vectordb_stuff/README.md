# vectordb_stuff

`vectordb_stuff` contains a standalone Rust program that creates a database from PDF files in the `pdfs` directory. The program reads the PDF files, processes them, and creates embeddings that are stored in a database.


1. Make sure you have the [rust toolchain and cargo](https://www.rust-lang.org/tools/install).
2. Optional: install `make` e.g. [Macos homebrew make](https://formulae.brew.sh/formula/make) or [linux make](https://askubuntu.com/questions/161104/how-do-i-install-make), or alternatively look at the `Makefile` for commands.

Then:

```
cd vectordb_stuff
make build
make help
make clear_database
make load_documents
```

You are now ready to go to the lambda_stuff directory.





