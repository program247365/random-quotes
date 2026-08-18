# random-quotes -- run `make help` for the full list of targets.

.DEFAULT_GOAL := help

PREFIX ?= $(HOME)/.kevin/bin
BIN    := random-quotes
CSV    := quotes.csv

.PHONY: help build release test fmt lint check run install uninstall clean

help: ## List every target this Makefile can run
	@printf 'random-quotes %s\n\n' "$$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml)"
	@printf 'USAGE\n  make <target>\n\n'
	@printf 'TARGETS\n'
	@grep -hE '^[a-zA-Z][a-zA-Z0-9_-]*:.*## ' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN { FS = ":.*## " } { printf "  %-10s %s\n", $$1, $$2 }'
	@printf '\nVARIABLES\n'
	@printf '  %-10s %s\n' 'PREFIX' 'Install directory (currently: $(PREFIX))'
	@printf '  %-10s %s\n' 'ARGS' 'Arguments passed through by `make run`'
	@printf '\nEXAMPLES\n'
	@printf '  make check                 Everything CI runs\n'
	@printf '  make install               Build and install to $(PREFIX)\n'
	@printf '  make install PREFIX=~/bin  Install somewhere else\n'
	@printf '  make run ARGS=--help       Run from source with arguments\n'

build: ## Compile a debug binary
	cargo build

release: ## Compile an optimized binary
	cargo build --release

test: ## Run the unit and CLI test suites
	cargo test

fmt: ## Format the source in place
	cargo fmt

lint: ## Lint with clippy, warnings treated as errors
	cargo clippy --all-targets -- -D warnings

check: ## Run everything CI runs: format check, lint, tests
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo test

run: ## Run from source, e.g. make run ARGS=--help
	@cargo run --quiet -- $(ARGS)

install: release ## Build and install the binary and quotes.csv into PREFIX
	@mkdir -p '$(PREFIX)'
	install -m 755 target/release/$(BIN) '$(PREFIX)/$(BIN)'
	install -m 644 $(CSV) '$(PREFIX)/$(CSV)'
	@printf '\nInstalled to %s:\n' '$(PREFIX)'
	@printf '  %s\n' '$(PREFIX)/$(BIN)' '$(PREFIX)/$(CSV)'
	@printf '\nVerifying:\n  '
	@'$(PREFIX)/$(BIN)' --version
	@printf '  '
	@'$(PREFIX)/$(BIN)'
	@case ":$$PATH:" in \
		*":$(PREFIX):"*) ;; \
		*) printf '\nWarning: %s is not on your PATH.\n' '$(PREFIX)' ;; \
		esac

uninstall: ## Remove the installed binary and quotes.csv from PREFIX
	rm -f '$(PREFIX)/$(BIN)' '$(PREFIX)/$(CSV)'
	@printf 'Removed %s and %s\n' '$(PREFIX)/$(BIN)' '$(PREFIX)/$(CSV)'

clean: ## Delete build artifacts
	cargo clean
