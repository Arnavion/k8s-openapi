#!/bin/bash

set -euo pipefail

./ci/rustup.sh

case "$OP" in
	'clippy')
		pushd k8s-openapi-codegen-common
		cargo clippy
		popd
		;;

	*)
		exit 1
		;;
esac
