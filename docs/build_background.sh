#!/bin/bash

## Needs when-changed, which can be installed via pip

when-changed *.tex "./build.sh '%f'"
