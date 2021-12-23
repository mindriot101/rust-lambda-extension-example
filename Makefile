all: package

.PHONY: package
package: extension.zip

extension.zip: target/x86_64-unknown-linux-musl/release/rust-lambda-extension-testing Makefile
	rm -f $@
	mkdir -p scratch/bin
	cp $< scratch/bin
	strip scratch/bin/rust-lambda-extension-testing
	(cd scratch && zip -r ../$@ bin)

target/x86_64-unknown-linux-musl/release/rust-lambda-extension-testing: $(wildcard src/*.rs)
	cargo build --release --target x86_64-unknown-linux-musl
