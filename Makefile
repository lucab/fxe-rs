REPO ?= quay.io/lucab/fxe
TAG ?= latest

.PHONY: all
all: target/x86_64-unknown-linux-musl/release/fxe .docker-stamp

target/x86_64-unknown-linux-musl/release/fxe: Cargo.toml src/main.rs
	cargo build --release --target x86_64-unknown-linux-musl

.docker-stamp: target/x86_64-unknown-linux-musl/release/fxe Dockerfile
	docker build -t ${REPO}:${TAG} .
	@touch .docker-stamp

.PHONY: run
run:
	docker run --privileged --pid=host -v /proc/1/ns/:/ns ${REPO}:${TAG}

.PHONY: clean
clean:
	@cargo clean
	@rm -f .docker-stamp
