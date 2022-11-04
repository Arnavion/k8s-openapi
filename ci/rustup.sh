#!/bin/bash

set -euo pipefail

# CI VMs have rustup and stable pre-installed, but they're not necessarily the latest.
# So expect them to exist but update them.

export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"

rustup self update

rustup set profile minimal

rustup update --no-self-update stable

rustup component add clippy

# Saves a few seconds for large crates
export CARGO_INCREMENTAL=0
