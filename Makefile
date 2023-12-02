.PHONY: check run test

# Find all folders starting with "day"
DAY_FOLDERS := $(wildcard day*)

check:
	@for folder in $(DAY_FOLDERS); do \
		echo "Checking $$folder..."; \
		cargo clippy --manifest-path $$folder/Cargo.toml -- -W clippy::pedantic; \
	done

run:
	@for folder in $(DAY_FOLDERS); do \
		echo "Running $$folder..."; \
		cargo run --release --manifest-path $$folder/Cargo.toml; \
	done

test:
	@for folder in $(DAY_FOLDERS); do \
		echo "Testing $$folder..."; \
		cargo test --manifest-path $$folder/Cargo.toml; \
	done