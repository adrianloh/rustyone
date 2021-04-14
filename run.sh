#!/bin/bash

# Clean the `target` directory
cargo clean

# Build everything in `src`
cargo build

# Build the main module
cargo build --release --bin rustyone

# Build and run a file contained in `src/bin`
cargo run --bin queue

# Build and run a file contained in `src/bin` and pass arguments
cargo run --bin csvparse testfiles/100rows.csv