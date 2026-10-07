.PHONY: check fmt clippy test format
check: fmt clippy test
fmt:
	cargo fmt --check
clippy:
	cargo clippy --all-targets --all-features -- -D warnings
test:
	cargo test
format:
	cargo fmt
