INPUT_DIRECTORY=${CURDIR}/countries
OUTPUT_DIRECTORY=${CURDIR}
OUTPUT_SRC_DIRECTORY=${OUTPUT_DIRECTORY}/src
CODE_GENERATOR_DIRECTORY=${CURDIR}/keshvar-code-generator/

build:
	@ echo "Building without default features..."
	cargo build --no-default-features
	@ ls -lash target/debug/*keshvar*
	@ echo "Building with default features..."
	cargo build
	@ ls -lash target/debug/*keshvar*
	@ echo "Building with 'geo' feature..."
	cargo build --features "geo"
	@ ls -lash target/debug/*keshvar*
	@ echo "Building with 'translations' feature..."
	cargo build --features "translations"
	@ ls -lash target/debug/*keshvar*
	@ echo "Building with 'subdivisions' feature..."
	cargo build --features "subdivisions"
	@ ls -lash target/debug/*keshvar*
	@ echo "Building with all features..."
	cargo build --no-default-features --all-features
	@ ls -lash target/debug/*keshvar*

generate:
	@ echo "Removing auto-generated codes..."
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/countries/* && touch ${OUTPUT_SRC_DIRECTORY}/countries/mod.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/alpha2.rs && touch ${OUTPUT_SRC_DIRECTORY}/alpha2.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/alpha3.rs && touch ${OUTPUT_SRC_DIRECTORY}/alpha3.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/consts.rs && touch ${OUTPUT_SRC_DIRECTORY}/consts.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/region.rs && touch ${OUTPUT_SRC_DIRECTORY}/region.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/gec.rs && touch ${OUTPUT_SRC_DIRECTORY}/gec.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/ioc.rs && touch ${OUTPUT_SRC_DIRECTORY}/ioc.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/currency_code.rs && touch ${OUTPUT_SRC_DIRECTORY}/currency_code.rs
	@ rm -rf build.log
	@ echo "Generating new codes..."
	@ cd ${CODE_GENERATOR_DIRECTORY} && cargo run -- ${INPUT_DIRECTORY} ${OUTPUT_DIRECTORY}
	@ echo "Compiling newly generated codes"
	@ ${MAKE} build

check-style:
	@ echo "Checking code-generator style..."
	@ cd ${CODE_GENERATOR_DIRECTORY} && cargo fmt --check
	@ echo "Checking generator source style..."
	cargo fmt --check > /dev/null

test:
	@echo "Running tests..."
	cargo test --all-features

clippy:
	@echo "Running Clippy..."
	cargo clippy

.PHONY: build generate check-style test clippy
