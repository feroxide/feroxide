#!/bin/sh

## Rust-docs

# Clean old files
rm -R ./rust-docs

# Generate docs
cargo doc --all --no-deps --release --all-features --color always

# Copy docs to root directory
cp -R ./target/doc ./rust-docs


## Latex docs
cd docs
mkdir -p build

pdflatex -interaction nonstopmode -halt-on-error -output-directory build *.tex *.latex

mv build/*.pdf .

rm -r build

cd ..
