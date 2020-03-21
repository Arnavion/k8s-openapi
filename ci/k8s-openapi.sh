#!/bin/bash

set -euo pipefail

./ci/rustup.sh

# Saves a few seconds for large crates
export CARGO_INCREMENTAL=0

FEATURES="--features v${VERSION//./_}"
if [ "$WITHOUT_API_FEATURE" = 'yes' ]; then
	FEATURES="--no-default-features $FEATURES"
fi

case "$OP" in
	'clippy')
		cargo clippy $FEATURES -- -D warnings
		;;

	'doc')
		cargo doc --no-deps $FEATURES
		;;

	'lib-tests')
		RUST_BACKTRACE=full cargo test $FEATURES
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
