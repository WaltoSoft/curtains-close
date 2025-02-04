./target/release/curtains-close: $(wildcard src/**.rs)
	cargo build --release --all-features

.PHONY: curtains-close
curtains-close: ./target/release/curtains-close

.PHONY: completions
completions: curtains-close
	mkdir -p completions
	OUT_DIR=completions cargo run --package curtains_close_completions --bin curtains_close_completions

.PHONY: all
all: curtains-close completions

.PHONY: clean
clean:
	rm -rf ./target ./completions