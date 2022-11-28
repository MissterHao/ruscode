# This is a makefile just for maintainer who is using windows operation system as developing environment.
all: build


.PHONY: build
build: 
	set RUSTFLAGS=
	set LLVM_PROFILE_FILE=
	cargo build

.PHONY: run
run: build
	cargo run

.PHONY: lint
lint: 
	set RUSTFLAGS=
	set LLVM_PROFILE_FILE=
	cargo fmt --check -- --color always
	cargo clippy --all-targets -- -D warnings

.PHONY: clean
clean:
	del *.profraw

.PHONY: test
test:
	set LLVM_PROFILE_FILE=ruscode-%p-%m.profraw 
	set RUSTFLAGS=-Cinstrument-coverage
	cargo build
	cargo test
	grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/
	