#!/bin/bash

set -euo pipefail

session_name='k8s-openapi'
max_version='1.27'

if ! tmux has-session -t "=$session_name"; then
	cd "$(dirname "$0")"

	tmux new-session -d -s "$session_name" -n '0' 'exec bash'
	tmux select-pane -t "$session_name:0.0" -T 'git'

	tmux split-window -t "$session_name:0.0" -h -p 50 "echo 'clear; K8S_RECORD_=1 ./test.sh $max_version run-tests' && exec bash"
	tmux select-pane -t "$session_name:0.1" -T 'tests'

	tmux split-window -t "$session_name:0.0" -v -p 50 $'cd ./k8s-openapi-codegen && echo \'clear; cargo build && time bash -c "set -euo pipefail; cargo run --color always 2>&1 | ts" && cargo +stable clippy && echo "stable clippy OK" && cargo clippy\' && exec bash'
	tmux select-pane -t "$session_name:0.1" -T 'codegen'

	tmux split-window -t "$session_name:0.2" -v -p 50 "echo 'clear; ./ci/global.sh && K8S_OPENAPI_ENABLED_VERSION=$max_version ./ci/per_version.sh; echo \$?' && exec bash"
	tmux select-pane -t "$session_name:0.3"
fi

exec tmux attach-session -t "=$session_name"
