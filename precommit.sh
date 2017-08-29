#!/bin/bash

export TRAVIS_RUST_VERSION="nightly"

total_exit_code=0

echo "Setting Rust build to nightly"
rustup default nightly
total_exit_code=$((total_exit_code + $?))

echo "Updating rust"
rustup update
total_exit_code=$((total_exit_code + $?))

echo "Updating Cargo.toml"
cargo update
total_exit_code=$((total_exit_code + $?))

echo "Formatting code"
cargo fmt
total_exit_code=$((total_exit_code + $?))

echo "Running clippy"
./clippy.sh test
total_exit_code=$((total_exit_code + $?))

echo "Running tests"
cargo test
total_exit_code=$((total_exit_code + $?))

echo "Generating documentation"
./generate_docs.sh
total_exit_code=$((total_exit_code + $?))


if [ $total_exit_code -ne 0 ]; then
  echo "One or more steps failed. Please read the logs" >&2
else
  echo "All steps completed succesfully. You're now ready to commit!" >&2
fi

exit $total_exit_code
