#!/bin/bash

set -euo pipefail

./ci/rustup.sh

case "$OP" in
	'clippy')
		pushd k8s-openapi-codegen
		cargo clippy
		popd
		;;

	'run')
		pushd k8s-openapi-codegen
		cargo run
		popd

		[ -z "$(git status --porcelain)" ]

		;;

	*)
		exit 1
		;;
esac
