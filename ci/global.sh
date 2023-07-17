#!/bin/bash

set -euo pipefail

. ./ci/rustup.sh

echo '### k8s-openapi-codegen-common:clippy ###'
pushd k8s-openapi-codegen-common
cargo test
cargo clippy -- -D warnings
popd

echo '### k8s-openapi-codegen:clippy ###'
pushd k8s-openapi-codegen
cargo clippy
popd

echo '### k8s-openapi-codegen:run ###'
pushd k8s-openapi-codegen
cargo run
popd
if [ -n "$(git status --porcelain)" ]; then
	echo "The changes to the generated code have not been git-add'ed."
	exit 1
fi

echo '### k8s-openapi-derive:clippy ###'
pushd k8s-openapi-derive
cargo clippy -- -D warnings
popd

echo '### style ###'
find . '(' -path './.git' -o -path './target*' -o -path './k8s-openapi-codegen-common/templates' ')' -prune -o -type f -print0 |
	while IFS= read -r -d '' f; do
		printf '%s: ' "$f"
		if [[ "$(tail -c 1 "$f" | wc -l)" -eq '0' ]]; then
			echo 'missing newline'
			exit 1
		fi
		echo 'OK'
	done
