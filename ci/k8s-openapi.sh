#!/bin/bash

set -euo pipefail

./ci/rustup.sh

# Saves a few seconds for large crates
export CARGO_INCREMENTAL=0

case "$OP" in
	'clippy')
		cargo clippy --features "v${VERSION//./_}" -- -D warnings
		;;

	'doc')
		cargo doc --no-deps --features "v${VERSION//./_}"
		;;

	'lib-tests')
		RUST_BACKTRACE=full cargo test --features "v${VERSION//./_}"
		;;

	'tests')
		pushd k8s-openapi-tests
		RUST_BACKTRACE=full ./test.sh run-tests "$VERSION"
		popd
		;;

	*)
		exit 1
		;;
esac
