#!/bin/bash

set -euo pipefail

./ci/rustup.sh

case "$OP" in
	'clippy')
		pushd k8s-openapi-tests
		cargo clippy --tests --features "test_v${VERSION//./_}"
		popd

		pushd k8s-openapi-tests-macro-deps
		cargo clippy --tests --features "test_v${VERSION//./_}"
		popd
		;;

	*)
		exit 1
		;;
esac
