INPUT_DIRECTORY=${CURDIR}/countries
OUTPUT_DIRECTORY=${CURDIR}
OUTPUT_SRC_DIRECTORY=${OUTPUT_DIRECTORY}/src
CODE_GENERATOR_DIRECTORY=${CURDIR}/keshvar-code-generator/

build:
	@ echo && echo "Building without default features..."
	cargo build --no-default-features
	@ ls -sh target/debug/*keshvar*
	@ echo && echo "Building with default features..."
	cargo build
	@ ls -sh target/debug/*keshvar*
	@ echo && echo "Building with 'geo' feature..."
	cargo build --features "geo"
	@ ls -sh target/debug/*keshvar*
	@ echo && echo "Building with 'translations' feature..."
	cargo build --features "translations"
	@ ls -sh target/debug/*keshvar*
	@ echo && echo "Building with 'subdivisions' feature..."
	cargo build --features "subdivisions"
	@ ls -sh target/debug/*keshvar*
	@ echo && echo "Building with all features..."
	cargo build --no-default-features --all-features
	@ ls -sh target/debug/*keshvar*

generate:
	@ echo "Removing auto-generated codes..."
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/countries/* && touch ${OUTPUT_SRC_DIRECTORY}/countries/mod.rs
	@ rm -rf ${OUTPUT_SRC_DIRECTORY}/search/* && touch ${OUTPUT_SRC_DIRECTORY}/search/mod.rs
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
	@ echo "Checking generated source style..."
	cargo fmt --check > /dev/null

format-style:
	@ echo "Formatting code-generator style..."
	@ cd ${CODE_GENERATOR_DIRECTORY} && cargo fmt
	@ echo "Formatting generated source style..."
	cargo fmt

test:
	@echo "Running tests..."
	cargo test --all-features

clippy:
	@echo "Running Clippy..."
	cargo clippy --all-features

.PHONY: build generate check-style format-style test clippy
