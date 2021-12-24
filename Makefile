LAYER_NAME := rust-lambda-extension-testing

all: help

.PHONY: help
help:
	@echo "Commands: package, upload"

.PHONY: package
package: extension.zip

extension.zip: target/x86_64-unknown-linux-musl/release/${LAYER_NAME} Makefile
	rm -f $@
	mkdir -p scratch/extensions
	cp $< scratch/extensions
	strip scratch/extensions/${LAYER_NAME}
	(cd scratch && zip -r ../$@ extensions)

target/x86_64-unknown-linux-musl/release/${LAYER_NAME}: $(wildcard src/*.rs) Cargo.lock Cargo.toml
	cargo build --release --target x86_64-unknown-linux-musl

.PHONY: upload
upload: extension.zip
	aws lambda publish-layer-version --layer-name ${LAYER_NAME} --zip-file fileb://extension.zip
