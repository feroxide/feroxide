#!/bin/sh

x=0

for f in `find src -type f | grep -v "data_atoms.rs\|atoms.rs.bak"`; do
  x=$(($x + $(wc -l < "$f")));
done

echo "Lines of code: $x"
