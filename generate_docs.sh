#!/bin/sh

cargo doc --all --no-deps --release --all-features --color always
cp -R ./target/doc/feroxide ./docs
