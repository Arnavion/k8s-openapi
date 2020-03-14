#!/bin/bash

set -euo pipefail

# CI VMs have rustup and stable pre-installed, but they're not necessarily the latest.
# So expect them to exist but update them.

rustup self update

rustup set profile minimal

rustup update --no-self-update stable
rustup default stable

rustup component add clippy
