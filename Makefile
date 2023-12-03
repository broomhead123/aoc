
# Find all folders starting with "day"
DAY_FOLDERS := $(wildcard day*)

.PHONY: check run test $(DAY_FOLDERS)

# Define rules for each folder
define FOLDER_RULES
check_$1: $1
	@echo "Checking $$@..."
	cargo clippy --manifest-path $$^/Cargo.toml -- -W clippy::pedantic;

run_$1: $1
	@echo "Running $$@..."
	cargo run --release --manifest-path $$^/Cargo.toml

test_$1: $1
	@echo "Testing $$@..."
	cargo test --manifest-path $$^/Cargo.toml

perf_$1: $1
	@echo "Perfing $$@..."
	cargo build  --manifest-path $$^/Cargo.toml
	PERF=/usr/lib/linux-tools/5.15.0-89-generic/perf flamegraph -F 1000 -- $$^/target/debug/$$^

endef

run: $(addprefix run_,$(DAY_FOLDERS))

check: $(addprefix check_,$(DAY_FOLDERS))

test: $(addprefix test_,$(DAY_FOLDERS))

perf: $(addprefix perf_,$(DAY_FOLDERS))

# Create rules for each folder
$(foreach folder,$(DAY_FOLDERS),$(eval $(call FOLDER_RULES,$(folder))))

