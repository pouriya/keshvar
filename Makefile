
build:
	cargo build --all-features

code_gen:
	@echo "Removing auto-generated codes..."
	@rm -rf src/countries/* && touch src/countries/mod.rs
	@rm -rf src/alpha2.rs && touch src/alpha2.rs
	@rm -rf src/alpha3.rs && touch src/alpha3.rs
	@rm -rf src/consts.rs && touch src/consts.rs
	@rm -rf src/region.rs && touch src/region.rs
	@rm -rf src/gec.rs && touch src/gec.rs
	@rm -rf src/ioc.rs && touch src/ioc.rs
	@cp Cargo.default.toml Cargo.toml
	@rm -rf build.log
	@echo "Generate & Compile new codes..."
	KESHVAR_GENERATE=1 ${MAKE} build
	@echo "Generate & Compile new codes with newly generated Cargo.toml"
	@echo ${MAKE} build
	${MAKE} fmt lint test

fmt:
	@echo "Formatting codes..."
	cargo fmt

test:
	@ echo "Building without any feature..."
	cargo build --no-default-features
	@ echo "Building with default features..."
	cargo build
	@ echo "Building with all features..."
	cargo build --no-default-features --all-features
	@echo "Running tests..."
	cargo test --all-features

lint:
	@echo "Running Clippy..."
	cargo clippy

.PHONY: build code_gen test lint
