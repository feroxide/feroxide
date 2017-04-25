#!/bin/sh

# Needs when-changed and pdflatex
## when-changed can be installed via pip
## pdflatex is available on the Linux repos

when-changed *.tex  "pdflatex -interaction nonstopmode -halt-on-error -output-directory build %f"
