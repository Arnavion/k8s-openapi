#!/bin/bash

set -euo pipefail

. ./ci/rustup.sh

export CARGO_TARGET_DIR="$PWD/target-tests-v$K8S_OPENAPI_ENABLED_VERSION"

for api_feature in 'yes' 'no'; do
    case "$api_feature" in
        'yes') features_args='';;
        'no') features_args='--no-default-features';;
    esac

    echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:${api_feature}:lib-tests ###"
    RUST_BACKTRACE=full cargo test $features_args

    echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:${api_feature}:clippy ###"
    cargo clippy $features_args -- -D warnings

    echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:${api_feature}:doc ###"
    RUSTDOCFLAGS='-D warnings' cargo doc --no-deps $features_args
done

echo "### k8s-openapi:${K8S_OPENAPI_ENABLED_VERSION}:tests ###"
RUST_BACKTRACE=full ./test.sh "$K8S_OPENAPI_ENABLED_VERSION" run-tests


echo '### k8s-openapi-tests:clippy ###'
test_version_feature_arg="--features test_v${K8S_OPENAPI_ENABLED_VERSION//./_}"
pushd k8s-openapi-tests
cargo clippy --tests $test_version_feature_arg
popd
pushd k8s-openapi-tests-macro-deps
cargo clippy --tests $test_version_feature_arg
popd
