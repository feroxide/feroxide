#!/bin/sh

# Dirty approach
cat src/atoms0.rs | tr '\n' '\f' | sed 's/{\f/{/g' | tr '\f' '\n' | sed -E 's/pub const ([A-Z]*).*?number: ([0-9]*).*?mass: ([0-9\.]*).*?symbol: (\"[A-Za-z]*\").*?name: (\"[a-z]*\").*?is_diatomic: (true|false).*?/[atoms.\1]\nnumber = \2\nmass = \3\nsymbol = \4\nname = \5\ndiatomic = \6\n/g' > src/atoms.toml
