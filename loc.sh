#!/bin/sh

## Just a small tool which I use to count the amount of LoC


loc=0

for f in `find src -type f | grep -v "data_atoms.rs\|atoms.rs.bak"`; do
  loc=$(($loc + $(wc -l < "$f")));
done

echo "Lines of code: $loc"
