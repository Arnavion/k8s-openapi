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

		if [ -n "$(git status --porcelain)" ]; then
			echo "The changes to the generated code have not been git-add'ed."
			exit 1
		fi

		;;

	*)
		exit 1
		;;
esac
