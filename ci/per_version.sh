#!/bin/bash

set -euo pipefail

. ./ci/rustup.sh

export CARGO_TARGET_DIR="$PWD/target-tests-v$K8S_OPENAPI_ENABLED_VERSION"

echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:lib-tests ###"
RUST_BACKTRACE=full cargo test

echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:clippy ###"
cargo clippy -- -D warnings

echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:doc ###"
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps

echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:tests ###"
RUST_BACKTRACE=full ./test.sh "$K8S_OPENAPI_ENABLED_VERSION" run-tests

echo '### k8s-openapi-tests:clippy ###'
pushd k8s-openapi-tests
cargo clippy --tests --features "test_v${K8S_OPENAPI_ENABLED_VERSION//./_}"
popd
pushd k8s-openapi-tests-macro-deps
cargo clippy --tests --features "test_v${K8S_OPENAPI_ENABLED_VERSION//./_}"
popd
