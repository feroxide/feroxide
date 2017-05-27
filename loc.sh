#!/bin/bash

## Just a small tool which I use to count the amount of LoC


loc_total=0

for f in `git ls-files | grep -Ev "rust-docs|\.pdf"`; do
  lines=$(wc -l < "$f")
  echo "$f: $lines"

  loc_total=$(($loc_total + $lines));
done

echo ""
echo "Total lines of code: $loc_total"
