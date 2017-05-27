#!/bin/bash

mode="$1"

if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
  if [ "$mode" = "install" ]; then
    cargo install clippy --verbose
    exit $?
  elif [ "$mode" = "test" ]; then
    cargo clippy --verbose -- -D warnings
    exit $?
  else
    echo "Mode not specified or unknown" >&2
    exit 1
  fi
else
  exit 0
fi
