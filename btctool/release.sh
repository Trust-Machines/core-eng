#!/usr/bin/env bash

set -ex

#VERSION=$(shell python -c "import btctool; print(btctool.__version__)")
VERSION=0.0.1
gh release create "v$VERSION" \
  --title "btctool $VERSION" \
  --latest \
  'btctool.bin#btctool (Mac M1)'
