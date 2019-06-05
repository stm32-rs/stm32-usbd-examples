#!/usr/bin/env bash

set -euxo pipefail

rustup target add thumbv6m-none-eabi
rustup target add thumbv7m-none-eabi
rustup target add thumbv7em-none-eabihf
