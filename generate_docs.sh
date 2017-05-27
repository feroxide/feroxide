#!/bin/bash


build_latex_docs() {
  cd docs
  ./build.sh *.tex
  cd ..
}


build_rust_docs() {
  # Clean old files, if available
  if [ -e rust-docs ]; then
    rm -r rust-docs
  fi
  if [ -e target/doc ]; then
    rm -r target/doc
  fi

  # Generate docs
  cargo doc --all --no-deps --release --all-features --color always

  # Copy docs to root directory
  cp -R ./target/doc ./rust-docs
}


echo ""
read -p ">> Do you want to (re)generate Latex docs? [Yn]" yn
case $yn in
    [Nn]* ) ;;
        * ) build_latex_docs;;
esac


echo ""
read -p ">> Do you want to (re)generate Rust docs? [Yn]" yn
case $yn in
    [Nn]* ) ;;
        * ) build_rust_docs;;
esac

exit 0
