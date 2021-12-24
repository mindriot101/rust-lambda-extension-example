all: package

.PHONY: package
package: extension.zip

extension.zip: target/x86_64-unknown-linux-musl/release/rust-lambda-extension-testing Makefile
	rm -f $@
	mkdir -p scratch/extensions
	cp $< scratch/extensions
	strip scratch/extensions/rust-lambda-extension-testing
	(cd scratch && zip -r ../$@ extensions)

target/x86_64-unknown-linux-musl/release/rust-lambda-extension-testing: $(wildcard src/*.rs) Cargo.lock Cargo.toml
	cargo build --release --target x86_64-unknown-linux-musl
