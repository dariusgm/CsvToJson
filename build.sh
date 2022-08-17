#!/usr/bin/env bash
cargo fmt
cargo clippy --no-deps
cargo test
cargo build --release
