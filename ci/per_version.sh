#!/bin/bash

set -euo pipefail

. ./ci/rustup.sh

export CARGO_TARGET_DIR="$PWD/target-tests-v$VERSION"

version_feature_arg="--features v${VERSION//./_}"
for api_feature in 'yes' 'no'; do
	case "$api_feature" in
		'yes') features_args="$version_feature_arg";;
		'no') features_args="--no-default-features $version_feature_arg";;
	esac

	echo "### k8s-openapi:${VERSION}:${api_feature}:lib-tests ###"
	RUST_BACKTRACE=full cargo test $features_args

	echo "### k8s-openapi:${VERSION}:${api_feature}:clippy ###"
	cargo clippy $features_args -- -D warnings

	echo "### k8s-openapi:${VERSION}:${api_feature}:doc ###"
	RUSTDOCFLAGS='-D warnings' cargo doc --no-deps $features_args
done

echo "### k8s-openapi:${VERSION}:tests ###"
RUST_BACKTRACE=full ./test.sh "$VERSION" run-tests


echo '### k8s-openapi-tests:clippy ###'
version_feature_arg="--features test_v${VERSION//./_}"
pushd k8s-openapi-tests
cargo clippy --tests $version_feature_arg
popd
pushd k8s-openapi-tests-macro-deps
cargo clippy --tests $version_feature_arg
popd
