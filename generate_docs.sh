#!/bin/sh

## Latex docs
cd docs
mkdir -p build

pdflatex -interaction nonstopmode -halt-on-error -output-directory build *.tex *.latex

mv build/*.pdf .

rm -r build

cd ..


while true; do
    read -p "Do you want to regenerate Rust docs too? [Yn]" yn
    case $yn in
        [Yy]* ) break;;
        [Nn]* ) exit;;
    esac
done


## Rust-docs

# Clean old files
rm -R ./rust-docs

# Generate docs
cargo doc --all --no-deps --release --all-features --color always

# Copy docs to root directory
cp -R ./target/doc ./rust-docs
