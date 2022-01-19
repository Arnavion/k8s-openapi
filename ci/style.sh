#!/bin/bash

set -euo pipefail

find . '(' -path './.git' -o -path './target*' -o -path './k8s-openapi-codegen-common/templates' ')' -prune -o -type f -print0 |
	while IFS= read -r -d '' f; do
		printf '%s: ' "$f"
		if [[ "$(tail -c 1 "$f" | wc -l)" -eq '0' ]]; then
			echo 'missing newline'
			exit 1
		fi
		echo 'OK'
	done
