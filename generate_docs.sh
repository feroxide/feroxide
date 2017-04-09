#!/bin/sh

# Clean old files
rm -R ./docs

# Generate docs
cargo doc --all --no-deps --release --all-features --color always

# Copy docs to root
cp -R ./target/doc ./docs
