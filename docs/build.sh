#!/bin/bash

# needs pdflatex
## pdflatex is available on the Linux repos

# loops over all arguments
while [ $# -gt 0 ]; do
  file="$1"
  shift

  # makes sure build folder exists
  mkdir -p build

  # builds LaTeX
  pdflatex -interaction nonstopmode -halt-on-error -output-directory build "$file"

  # moves output PDF files to main folder
  if [ $? -eq 0 ]; then
    mv build/*.pdf .
  fi
done
