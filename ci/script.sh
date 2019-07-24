#!/usr/bin/env bash

REPO=$(dirname $(readlink -f $0))/..
cd "$REPO"

set -euxo pipefail

for pkg in $(ls -d example-*); do
    cd "$REPO/$pkg"
    cargo build --release
    cargo build --release --examples
done
